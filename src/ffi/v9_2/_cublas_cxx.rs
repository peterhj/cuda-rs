/* automatically generated by rust-bindgen */

extern "C" {
    pub fn cublasHgemm(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const __half,
        A: *const __half,
        lda: ::std::os::raw::c_int,
        B: *const __half,
        ldb: ::std::os::raw::c_int,
        beta: *const __half,
        C: *mut __half,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasHgemmBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const __half,
        Aarray: *const *const __half,
        lda: ::std::os::raw::c_int,
        Barray: *const *const __half,
        ldb: ::std::os::raw::c_int,
        beta: *const __half,
        Carray: *const *mut __half,
        ldc: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasHgemmStridedBatched(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const __half,
        A: *const __half,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        B: *const __half,
        ldb: ::std::os::raw::c_int,
        strideB: ::std::os::raw::c_longlong,
        beta: *const __half,
        C: *mut __half,
        ldc: ::std::os::raw::c_int,
        strideC: ::std::os::raw::c_longlong,
        batchCount: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
