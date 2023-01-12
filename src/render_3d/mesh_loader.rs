use std::error;

use crate::utils::StrUtils;

use super::*;

pub trait MeshLoader {
    type Error: error::Error;

    fn load(data: &[u8]) -> Result<Mesh, Self::Error>;
}

#[derive(Debug)]
pub enum ObjError {
    NgonError{line: usize, vertex_count: usize},
    InvalidVertexIndex { line: usize, index: usize },
    InvalidNormalIndex { line: usize, index: usize },
    InvalidZeroIndex { line: usize },
    VertexNormalIndexMismatch { line: usize },
    InvalidFloat { line: usize, string: String },
    InvalidInt { line: usize, string: String },
    ParseError { line: usize },
    InvalidUtf8 { index: usize },
}

impl Display for ObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjError::NgonError{line, vertex_count} => {
                write!(
                    f,
                    "mesh data contained an n-gon with {} corner(s) at line {}. this parser only supports triangulated faces",
                    vertex_count,
                    line,
                )
            }
            ObjError::InvalidVertexIndex { line, index } => {
                write!(
                    f,
                    "reference to a non-existant vertex index \"{}\" at line {}",
                    index,
                    line
                )
            }
            ObjError::InvalidNormalIndex { line, index } => {
                write!(
                    f,
                    "reference to a non-existant vertex normal index \"{}\" at line {}",
                    index,
                    line,
                )
            }
            ObjError::VertexNormalIndexMismatch { line } => {
                write!(
                    f,
                    "some face vertices has vertex normals while others don't on line {}",
                    line
                )
            }
            ObjError::InvalidZeroIndex { line } => {
                write!(
                    f,
                    "reference with index \"0\" on line {}. indexes start at \"1\"",
                    line
                )
            }
            ObjError::InvalidFloat { line, string } => {
                write!(f, "invalid float \"{}\" at line {}", string, line)
            }
            ObjError::InvalidInt { line, string } => {
                write!(f, "invalid int \"{}\" at line {}", string, line)
            }
            ObjError::ParseError { line } => {
                write!(f, "input contained invalid syntax at line {}", line)
            }
            ObjError::InvalidUtf8 { index } => {
                write!(f, "invalid utf-8 at byte index {}", index)
            }
        }
    }
}

impl error::Error for ObjError {}

pub struct ObjMeshLoader {}

impl MeshLoader for ObjMeshLoader {
    type Error = ObjError;

    fn load(data: &[u8]) -> Result<Mesh, Self::Error> {
        // source: https://en.wikipedia.org/wiki/Wavefront_.obj_file
        
        let text: &str;

        match std::str::from_utf8(data) {
            Ok(parsed) => text = parsed,
            Err(error) => {
                return Err(ObjError::InvalidUtf8 {
                    index: error.valid_up_to(),
                })
            }
        }

        struct VertexDeclaration {
            pos: Vec3,
        }

        struct NormalDeclaration {
            normal: Vec3,
        }

        struct FaceDeclaration {
            vertices: (usize, usize, usize),
            normals: Option<(usize, usize, usize)>,
            line: usize,
        }

        let mut vertex_declarations = Vec::new();
        let mut normal_declarations = Vec::new();
        let mut face_declarations = Vec::new();

        for (i, line) in text.lines().enumerate() {
            let mut words = line.words();

            match words.next() {
                Some("v") => {
                    let components = words
                        .map(|string| -> Result<f32, ObjError> {
                            let str = string.trim();
                            if let Ok(num) = str.parse::<f32>() {
                                Ok(num)
                            } else {
                                Err(ObjError::InvalidFloat {
                                    line: i + 1,
                                    string: str.to_owned(),
                                })
                            }
                        })
                        .take(3)
                        .collect::<Result<Vec<f32>, ObjError>>()?;

                    if components.len() < 3 {
                        return Err(ObjError::ParseError { line: i + 1 });
                    }

                    vertex_declarations.push(VertexDeclaration {
                        pos: vec3(components[0], components[1], components[2]),
                    });
                }
                Some("vn") => {
                    let components = words
                        .map(|string| -> Result<f32, ObjError> {
                            let str = string.trim();
                            if let Ok(num) = str.parse::<f32>() {
                                Ok(num)
                            } else {
                                Err(ObjError::InvalidFloat {
                                    line: i + 1,
                                    string: str.to_owned(),
                                })
                            }
                        })
                        .take(3)
                        .collect::<Result<Vec<f32>, ObjError>>()?;

                    if components.len() < 3 {
                        return Err(ObjError::ParseError { line: i + 1 });
                    }

                    normal_declarations.push(NormalDeclaration {
                        normal: vec3(components[0], components[1], components[2]),
                    });
                }
                Some("f") => {
                    let components = words
                    .map(|string| {
                        let mut parts = string.split('/');

                        // vertex index
                        let Some(face) = parts.next() else {
                            return Err(ObjError::ParseError { line: i + 1 });
                        };
                        
                        let Ok(mut face) = face.parse::<usize>() else {
                            return Err(ObjError::InvalidInt { line: i + 1, string: face.to_owned() })
                        };
                        if face == 0 {
                            return Err(ObjError::InvalidZeroIndex { line: i + 1 });
                        }else {
                            face -= 1;
                        }

                        // ignore texture index
                        parts.next();

                        // normal index
                        let normal = match parts.next().map(|string| {
                            let result = string.parse::<usize>().map_err(|_| {
                                ObjError::InvalidInt { line: i + 1, string: string.to_owned() }
                            });

                            let num = result?;
                            if num == 0 {
                                Err(ObjError::InvalidZeroIndex { line: i + 1 })
                            } else {
                                Ok(num - 1)
                            }
                        }) {
                            None => None,
                            Some(res) => Some(res?),
                        };

                        Ok((face, normal))
                    })
                    .collect::<Result<Vec<_>, ObjError>>()?;

                    if components.len() < 3 || components.len() > 3 {
                        return Err(ObjError::NgonError { line: i + 1, vertex_count: components.len() })
                    }

                    let has_normal = components[0].1.is_some();
                    if !components.iter().skip(1).all(|(_, x)| x.is_some() == has_normal) {
                        return Err(ObjError::VertexNormalIndexMismatch { line: i + 1 })
                    }



                    face_declarations.push(FaceDeclaration {
                        line: i + 1,
                        normals: if has_normal {
                            Some((components[0].1.unwrap(),components[1].1.unwrap(),components[2].1.unwrap()))
                        } else {
                            None
                        },
                        vertices: (components[0].0, components[1].0, components[2].0)
                    })


                },
                _ => continue,
            }
        }

        let vertices = vertex_declarations.iter().map(|vertex| vertex.pos).collect::<Vec<_>>();

        let faces = face_declarations.iter().map(|face|{
            let len = vertex_declarations.len();
            if face.vertices.0 >= len {
                return Err(ObjError::InvalidVertexIndex { line: face.line, index: face.vertices.0 });
            }
            if face.vertices.1 >= len {
                return Err(ObjError::InvalidVertexIndex { line: face.line, index: face.vertices.1 });
            }
            if face.vertices.2 >= len {
                return Err(ObjError::InvalidVertexIndex { line: face.line, index: face.vertices.2 });
            }


            
            Ok(RefTriangle(Index::new(face.vertices.0), Index::new(face.vertices.1), Index::new(face.vertices.2)))
        })
        .collect::<Result<Vec<RefTriangle>, ObjError>>()?;

        let normals = face_declarations.iter().map(|face| -> Result<Option<(Vec3, Vec3, Vec3)>, ObjError> {
            let Some(normals) = face.normals else {
                return Ok(None);
            };

            let len = normal_declarations.len();

            if normals.0 >= len {
                return Err(ObjError::InvalidNormalIndex{ line: face.line, index: normals.0 });
            }
            if normals.1 >= len {
                return Err(ObjError::InvalidNormalIndex { line: face.line, index: normals.1 });
            }
            if normals.2 >= len {
                return Err(ObjError::InvalidNormalIndex { line: face.line, index: normals.2 });
            }

            Ok(Some((normal_declarations[normals.0].normal,normal_declarations[normals.1].normal,normal_declarations[normals.2].normal,)))
        }).collect::<Result<Option<Vec<(Vec3, Vec3, Vec3)>>, ObjError>>()?;

        // TODO: Add ability to parse vertex colors.
        Ok(Mesh::new(vertices, faces, normals, None))
    }
}
