
use types::{Matrix3x3, Matrix4x4, Quaternion};

//TODO implement these in rust
extern {
    ///  Construct a quaternion from a 3x3 rotation matrix.
    ///
    /// * param quat Receives the output quaternion.
    /// * param mat Matrix to 'quaternionize'.
    ///
    /// * see aiQuaternion(const aiMatrix3x3& pRotMatrix)
    ///
    // ASSIMP_API void aiCreateQuaternionFromMatrix(
    //     C_STRUCT aiQuaternion* quat,
    //     const C_STRUCT aiMatrix3x3* mat);
    pub fn aiCreateQuaternionFromMatrix(quat: *mut Quaternion, mat: *const Matrix3x3);

        // /** Decompose a transformation matrix into its rotational, translational and
        // *  scaling components.
        // *
        // * @param mat Matrix to decompose
        // * @param scaling Receives the scaling component
        // * @param rotation Receives the rotational component
        // * @param position Receives the translational component.
        // * @see aiMatrix4x4::Decompose (aiVector3D&, aiQuaternion&, aiVector3D&) const;
        // */
        // // ASSIMP_API void aiDecomposeMatrix(
        // //     const C_STRUCT aiMatrix4x4* mat,
        // //     C_STRUCT aiVector3D* scaling,
        // //     C_STRUCT aiQuaternion* rotation,
        // //     C_STRUCT aiVector3D* position);


    /// Transpose a 3x3 matrix.
    /// *param mat Pointer to the matrix to be transposed
    ///
    // ASSIMP_API void aiTransposeMatrix3( C_STRUCT aiMatrix3x3* mat);
    pub fn aiTransposeMatrix3(mat: *mut Matrix3x3);

    ///  Transpose a 4x4 matrix.
    /// *param mat Pointer to the matrix to be transposed
    ///
    // ASSIMP_API void aiTransposeMatrix4( C_STRUCT aiMatrix4x4* mat);
    pub fn aiTransposeMatrix4(mat: *mut Matrix4x4);

    // /** Transform a vector by a 3x3 matrix
    // *  @param vec Vector to be transformed.
    // *  @param mat Matrix to transform the vector with.
    // */
    // // ASSIMP_API void aiTransformVecByMatrix3(
    // //     C_STRUCT aiVector3D* vec,
    // //     const C_STRUCT aiMatrix3x3* mat);

    // /** Transform a vector by a 4x4 matrix
    // *  @param vec Vector to be transformed.
    // *  @param mat Matrix to transform the vector with.
    // */
    // // ASSIMP_API void aiTransformVecByMatrix4(
    // //     C_STRUCT aiVector3D* vec,
    // //     const C_STRUCT aiMatrix4x4* mat);

    ///  Multiply two 4x4 matrices.
    /// * param dst First factor, receives result.
    /// * param src Matrix to be multiplied with 'dst'.
    // ASSIMP_API void aiMultiplyMatrix4(
    //     C_STRUCT aiMatrix4x4* dst,
    //     const C_STRUCT aiMatrix4x4* src);
    pub fn aiMultiplyMatrix4(dest: *mut Matrix4x4, src: *const Matrix4x4);

    ///  Multiply two 3x3 matrices.
    ///  * param dst First factor, receives result.
    ///  * param src Matrix to be multiplied with 'dst'.
    ///
    // ASSIMP_API void aiMultiplyMatrix3(
    //     C_STRUCT aiMatrix3x3* dst,
    //     const C_STRUCT aiMatrix3x3* src);
    pub fn aiMultiplyMatrix3(dest: *mut Matrix3x3, src: *const Matrix3x3);

    // /** Get a 3x3 identity matrix.//{{{
    // *  @param mat Matrix to receive its personal identity
    // */
    // // ASSIMP_API void aiIdentityMatrix3( C_STRUCT aiMatrix3x3* mat);

    // /** Get a 4x4 identity matrix.
    // *  @param mat Matrix to receive its personal identity
    // */
    // // ASSIMP_API void aiIdentityMatrix4( C_STRUCT aiMatrix4x4* mat);//}}}
}
