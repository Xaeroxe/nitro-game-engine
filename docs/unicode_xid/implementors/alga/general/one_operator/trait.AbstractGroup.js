(function() {var implementors = {};
implementors["nalgebra"] = ["impl&lt;N, R:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, C:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; for <a class='struct' href='nalgebra/core/struct.Matrix.html' title='nalgebra::core::Matrix'>Matrix</a>&lt;N, R, C, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='nalgebra/core/trait.Scalar.html' title='nalgebra::core::Scalar'>Scalar</a> + <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; + <a class='trait' href='num_traits/identities/trait.Zero.html' title='num_traits::identities::Zero'>Zero</a> + <a class='trait' href='alga/general/operator/trait.ClosedAdd.html' title='alga::general::operator::ClosedAdd'>ClosedAdd</a> + <a class='trait' href='alga/general/operator/trait.ClosedNeg.html' title='alga::general::operator::ClosedNeg'>ClosedNeg</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, R, C, S&gt;</span>","impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.RotationBase.html' title='nalgebra::geometry::RotationBase'>RotationBase</a>&lt;N, D, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, D&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, D, S&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.QuaternionBase.html' title='nalgebra::geometry::QuaternionBase'>QuaternionBase</a>&lt;N, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='type' href='nalgebra/geometry/type.UnitQuaternionBase.html' title='nalgebra::geometry::UnitQuaternionBase'>UnitQuaternionBase</a>&lt;N, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N:&nbsp;<a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='type' href='nalgebra/geometry/type.UnitComplex.html' title='nalgebra::geometry::UnitComplex'>UnitComplex</a>&lt;N&gt;","impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.TranslationBase.html' title='nalgebra::geometry::TranslationBase'>TranslationBase</a>&lt;N, D, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, S, R&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.IsometryBase.html' title='nalgebra::geometry::IsometryBase'>IsometryBase</a>&lt;N, D, S, R&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;R: <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/struct.PointBase.html' title='nalgebra::geometry::PointBase'>PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, S, R&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.SimilarityBase.html' title='nalgebra::geometry::SimilarityBase'>SimilarityBase</a>&lt;N, D, S, R&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;R: <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/struct.PointBase.html' title='nalgebra::geometry::PointBase'>PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>DimNameAdd</a>&lt;<a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;, S, C&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/struct.TransformBase.html' title='nalgebra::geometry::TransformBase'>TransformBase</a>&lt;N, D, S, C&gt; <span class='where fmt-newline'>where N: <a class='trait' href='nalgebra/core/trait.Scalar.html' title='nalgebra::core::Scalar'>Scalar</a> + <a class='trait' href='alga/general/specialized/trait.Field.html' title='alga::general::specialized::Field'>Field</a> + <a class='trait' href='approx/trait.ApproxEq.html' title='approx::ApproxEq'>ApproxEq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='type' href='nalgebra/core/dimension/type.DimNameSum.html' title='nalgebra::core::dimension::DimNameSum'>DimNameSum</a>&lt;D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;, <a class='type' href='nalgebra/core/dimension/type.DimNameSum.html' title='nalgebra::core::dimension::DimNameSum'>DimNameSum</a>&lt;D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;C: <a class='trait' href='nalgebra/geometry/trait.SubTCategoryOf.html' title='nalgebra::geometry::SubTCategoryOf'>SubTCategoryOf</a>&lt;<a class='enum' href='nalgebra/geometry/enum.TProjective.html' title='nalgebra::geometry::TProjective'>TProjective</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='type' href='nalgebra/core/dimension/type.DimNameSum.html' title='nalgebra::core::dimension::DimNameSum'>DimNameSum</a>&lt;D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;, <a class='type' href='nalgebra/core/dimension/type.DimNameSum.html' title='nalgebra::core::dimension::DimNameSum'>DimNameSum</a>&lt;D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;, S&gt;</span>",];
implementors["ncollide"] = ["impl&lt;N, R, C, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; for <a class='struct' href='nalgebra/core/matrix/struct.Matrix.html' title='nalgebra::core::matrix::Matrix'>Matrix</a>&lt;N, R, C, S&gt; <span class='where fmt-newline'>where C: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='nalgebra/core/scalar/trait.Scalar.html' title='nalgebra::core::scalar::Scalar'>Scalar</a> + <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; + <a class='trait' href='num_traits/identities/trait.Zero.html' title='num_traits::identities::Zero'>Zero</a> + <a class='trait' href='alga/general/operator/trait.ClosedAdd.html' title='alga::general::operator::ClosedAdd'>ClosedAdd</a>&lt;N&gt; + <a class='trait' href='alga/general/operator/trait.ClosedNeg.html' title='alga::general::operator::ClosedNeg'>ClosedNeg</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;R: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, R, C, S&gt;</span>","impl&lt;N, D, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/rotation/struct.RotationBase.html' title='nalgebra::geometry::rotation::RotationBase'>RotationBase</a>&lt;N, D, S&gt; <span class='where fmt-newline'>where D: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, D&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, D, S&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Additive.html' title='alga::general::operator::Additive'>Additive</a>&gt; for <a class='struct' href='nalgebra/geometry/quaternion/struct.QuaternionBase.html' title='nalgebra::geometry::quaternion::QuaternionBase'>QuaternionBase</a>&lt;N, S&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/core/unit/struct.Unit.html' title='nalgebra::core::unit::Unit'>Unit</a>&lt;<a class='struct' href='nalgebra/geometry/quaternion/struct.QuaternionBase.html' title='nalgebra::geometry::quaternion::QuaternionBase'>QuaternionBase</a>&lt;N, S&gt;&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/core/unit/struct.Unit.html' title='nalgebra::core::unit::Unit'>Unit</a>&lt;<a class='struct' href='num_complex/struct.Complex.html' title='num_complex::Complex'>Complex</a>&lt;N&gt;&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a></span>","impl&lt;N, D, S&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/translation/struct.TranslationBase.html' title='nalgebra::geometry::translation::TranslationBase'>TranslationBase</a>&lt;N, D, S&gt; <span class='where fmt-newline'>where D: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D, S, R&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/isometry/struct.IsometryBase.html' title='nalgebra::geometry::isometry::IsometryBase'>IsometryBase</a>&lt;N, D, S, R&gt; <span class='where fmt-newline'>where D: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;R: <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/point/struct.PointBase.html' title='nalgebra::geometry::point::PointBase'>PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D, S, R&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/similarity/struct.SimilarityBase.html' title='nalgebra::geometry::similarity::SimilarityBase'>SimilarityBase</a>&lt;N, D, S, R&gt; <span class='where fmt-newline'>where D: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;R: <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/point/struct.PointBase.html' title='nalgebra::geometry::point::PointBase'>PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>","impl&lt;N, D, S, C&gt; <a class='trait' href='alga/general/one_operator/trait.AbstractGroup.html' title='alga::general::one_operator::AbstractGroup'>AbstractGroup</a>&lt;<a class='struct' href='alga/general/operator/struct.Multiplicative.html' title='alga::general::operator::Multiplicative'>Multiplicative</a>&gt; for <a class='struct' href='nalgebra/geometry/transform/struct.TransformBase.html' title='nalgebra::geometry::transform::TransformBase'>TransformBase</a>&lt;N, D, S, C&gt; <span class='where fmt-newline'>where C: <a class='trait' href='nalgebra/geometry/transform/trait.SubTCategoryOf.html' title='nalgebra::geometry::transform::SubTCategoryOf'>SubTCategoryOf</a>&lt;<a class='enum' href='nalgebra/geometry/transform/enum.TProjective.html' title='nalgebra::geometry::transform::TProjective'>TProjective</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;D: <a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>DimNameAdd</a>&lt;<a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='nalgebra/core/scalar/trait.Scalar.html' title='nalgebra::core::scalar::Scalar'>Scalar</a> + <a class='trait' href='alga/general/specialized/trait.Field.html' title='alga::general::specialized::Field'>Field</a> + <a class='trait' href='approx/trait.ApproxEq.html' title='approx::ApproxEq'>ApproxEq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D::<a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>Output</a>, D::<a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>Output</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D::<a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>Output</a>, D::<a class='trait' href='nalgebra/core/dimension/trait.DimNameAdd.html' title='nalgebra::core::dimension::DimNameAdd'>Output</a>, S&gt;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
