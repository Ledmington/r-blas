#![forbid(unsafe_code)]

/// y = alpha * op(A) * x + beta * y, A general band matrix (s, d, c, z)
pub fn gbmv() {
    todo!()
}

/// y = alpha * op(A) * x + beta * y, A general matrix (s, d, c, z)
pub fn gemv() {
    todo!()
}

/// A = alpha * x * y^T + A, rank-1 update (s, d)
pub fn ger() {
    todo!()
}

/// A = alpha * x * y^H + A, conjugated rank-1 update (c, z)
pub fn gerc() {
    todo!()
}

/// A = alpha * x * y^T + A, unconjugated rank-1 update (c, z)
pub fn geru() {
    todo!()
}

/// y = alpha * A * x + beta * y, A Hermitian band matrix (c, z)
pub fn hbmv() {
    todo!()
}

/// y = alpha * A * x + beta * y, A Hermitian matrix (c, z)
pub fn hemv() {
    todo!()
}

/// A = alpha * x * x^H + A, Hermitian rank-1 update (c, z)
pub fn her() {
    todo!()
}

/// A = alpha * x * y^H + conj(alpha) * y * x^H + A, Hermitian rank-2 update (c, z)
pub fn her2() {
    todo!()
}

/// y = alpha * A * x + beta * y, A Hermitian packed matrix (c, z)
pub fn hpmv() {
    todo!()
}

/// A = alpha * x * x^H + A, Hermitian packed rank-1 update (c, z)
pub fn hpr() {
    todo!()
}

/// A = alpha * x * y^H + conj(alpha) * y * x^H + A, Hermitian packed rank-2 update (c, z)
pub fn hpr2() {
    todo!()
}

/// y = alpha * A * x + beta * y, A symmetric band matrix (s, d)
pub fn sbmv() {
    todo!()
}

/// y = alpha * A * x + beta * y, A symmetric packed matrix (s, d)
pub fn spmv() {
    todo!()
}

/// A = alpha * x * x^T + A, symmetric packed rank-1 update (s, d)
pub fn spr() {
    todo!()
}

/// A = alpha * x * y^T + alpha * y * x^T + A, symmetric packed rank-2 update (s, d)
pub fn spr2() {
    todo!()
}

/// y = alpha * A * x + beta * y, A symmetric matrix (s, d)
pub fn symv() {
    todo!()
}

/// A = alpha * x * x^T + A, symmetric rank-1 update (s, d)
pub fn syr() {
    todo!()
}

/// A = alpha * x * y^T + alpha * y * x^T + A, symmetric rank-2 update (s, d)
pub fn syr2() {
    todo!()
}

/// x = op(A) * x, A triangular band matrix (s, d, c, z)
pub fn tbmv() {
    todo!()
}

/// solves op(A) * x = b, A triangular band matrix (s, d, c, z)
pub fn tbsv() {
    todo!()
}

/// x = op(A) * x, A triangular packed matrix (s, d, c, z)
pub fn tpmv() {
    todo!()
}

/// solves op(A) * x = b, A triangular packed matrix (s, d, c, z)
pub fn tpsv() {
    todo!()
}

/// x = op(A) * x, A triangular matrix (s, d, c, z)
pub fn trmv() {
    todo!()
}

/// solves op(A) * x = b, A triangular matrix (s, d, c, z)
pub fn trsv() {
    todo!()
}
