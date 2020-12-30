use serde::Serialize;
use std::convert::TryInto;
use std::error::Error;

pub const EMPTY_STR: &str = "";

struct Matrix4x4 {}

impl Matrix4x4 {
    const ZERO: [[f32; 4]; 4] = [[0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4]];
    const IDENTITY: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Transform {
    pub r0: Vector4f,
    pub r1: Vector4f,
    pub r2: Vector4f,
    pub r3: Vector4f,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ColorMaterial {
    pub ambient: Color3,
    pub diffuse: Color3,
    pub emissive: Color3,
    pub glossiness: u8,
    pub opacity: u8,
    pub specular: Color3,
}

pub fn is_4x4_identity(transform: [[f32; 4]; 4]) -> bool {
    return transform == Matrix4x4::IDENTITY;
}

pub fn multiply_4x4(xs: [[f32; 4]; 4], ys: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result: [[f32; 4]; 4] = Matrix4x4::ZERO;
    for r in 0..4 {
        for c in 0..4 {
            for i in 0..4 {
                result[r][c] += xs[r][i] * ys[i][c];
            }
        }
    }

    return result;
}

pub fn to_arr_3<T>(v: Vec<T>) -> [T; 3] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected Vec of length {} but was {}", 3, v.len()))
}

pub fn to_arr_9<T>(v: Vec<T>) -> [T; 9] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected Vec of length {} but was {}", 9, v.len()))
}

pub fn to_float_arr(a: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let parts: Vec<&str> = a.split(",").collect();
    let mut fs: Vec<f32> = vec![];
    for n in parts {
        fs.push(n.parse()?);
    }

    return Ok(fs);
}

pub fn to_transform(t: [[f32; 4]; 4]) -> Transform {
    return Transform {
        r0: Vector4f {
            x: t[0][0],
            y: t[0][1],
            z: t[0][2],
            w: t[0][3],
        },
        r1: Vector4f {
            x: t[1][0],
            y: t[1][1],
            z: t[1][2],
            w: t[1][3],
        },
        r2: Vector4f {
            x: t[2][0],
            y: t[2][1],
            z: t[2][2],
            w: t[2][3],
        },
        r3: Vector4f {
            x: t[3][0],
            y: t[3][1],
            z: t[3][2],
            w: t[3][3],
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix4x4_zero() {
        assert_eq!([[0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4]], Matrix4x4::ZERO);
    }

    #[test]
    fn matrix4x4_identity() {
        assert_eq!(
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            Matrix4x4::IDENTITY
        );
    }

    #[test]
    fn false_if_not_identity() {
        assert_eq!(is_4x4_identity(Matrix4x4::ZERO), false);
    }

    #[test]
    fn true_if_identity() {
        assert_eq!(is_4x4_identity(Matrix4x4::IDENTITY), true);
    }

    #[test]
    fn multiply_zeros() {
        assert_eq!(
            multiply_4x4(Matrix4x4::ZERO, Matrix4x4::ZERO),
            Matrix4x4::ZERO
        );
    }

    #[test]
    fn multiply_identity() {
        assert_eq!(
            multiply_4x4(Matrix4x4::IDENTITY, Matrix4x4::IDENTITY),
            Matrix4x4::IDENTITY
        );
    }

    #[test]
    fn multiply() {
        assert_eq!(
            multiply_4x4(
                [
                    [2.0, 1.0, 0.0, 0.0],
                    [0.0, 3.0, 0.0, 0.0],
                    [0.0, 1.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
                [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 2.0, 0.0, 0.0],
                    [0.0, 7.0, 4.0, 0.0],
                    [0.0, 0.0, 0.0, 5.0],
                ]
            ),
            [
                [2.0, 2.0, 0.0, 0.0],
                [0.0, 6.0, 0.0, 0.0],
                [0.0, 9.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 5.0],
            ]
        );
    }
}
