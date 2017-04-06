(function() {var implementors = {};
implementors["nalgebra"] = ["impl&lt;N, D:&nbsp;<a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>, SA, SB&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/struct.PointBase.html' title='nalgebra::geometry::PointBase'>PointBase</a>&lt;N, D, SB&gt;&gt; for <a class='struct' href='nalgebra/geometry/struct.RotationBase.html' title='nalgebra::geometry::RotationBase'>RotationBase</a>&lt;N, D, SA&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, D&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, Alloc=SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, D, SA&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SB&gt;</span>","impl&lt;N, SA, SB&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/struct.PointBase.html' title='nalgebra::geometry::PointBase'>PointBase</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, SB&gt;&gt; for <a class='type' href='nalgebra/geometry/type.UnitQuaternionBase.html' title='nalgebra::geometry::UnitQuaternionBase'>UnitQuaternionBase</a>&lt;N, SA&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, Alloc=SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SA&gt; + <a class='trait' href='nalgebra/core/allocator/trait.Allocator.html' title='nalgebra::core::allocator::Allocator'>Allocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SB&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/struct.PointBase.html' title='nalgebra::geometry::PointBase'>PointBase</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, S&gt;&gt; for <a class='type' href='nalgebra/geometry/type.UnitComplex.html' title='nalgebra::geometry::UnitComplex'>UnitComplex</a>&lt;N&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>",];
implementors["ncollide"] = ["impl&lt;N, D, SA, SB&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/point/struct.PointBase.html' title='nalgebra::geometry::point::PointBase'>PointBase</a>&lt;N, D, SB&gt;&gt; for <a class='struct' href='nalgebra/geometry/rotation/struct.RotationBase.html' title='nalgebra::geometry::rotation::RotationBase'>RotationBase</a>&lt;N, D, SA&gt; <span class='where fmt-newline'>where D: <a class='trait' href='nalgebra/core/dimension/trait.DimName.html' title='nalgebra::core::dimension::DimName'>DimName</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, D&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, Alloc=SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, D, SA&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, D, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SB&gt;</span>","impl&lt;N, SA, SB&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/point/struct.PointBase.html' title='nalgebra::geometry::point::PointBase'>PointBase</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, SB&gt;&gt; for <a class='struct' href='nalgebra/core/unit/struct.Unit.html' title='nalgebra::core::unit::Unit'>Unit</a>&lt;<a class='struct' href='nalgebra/geometry/quaternion/struct.QuaternionBase.html' title='nalgebra::geometry::quaternion::QuaternionBase'>QuaternionBase</a>&lt;N, SA&gt;&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, Alloc=SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U4.html' title='nalgebra::core::dimension::U4'>U4</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SA&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.Allocator.html' title='nalgebra::core::allocator::Allocator'>Allocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U3.html' title='nalgebra::core::dimension::U3'>U3</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, SB&gt;</span>","impl&lt;N, S&gt; <a class='trait' href='alga/linear/transformation/trait.Rotation.html' title='alga::linear::transformation::Rotation'>Rotation</a>&lt;<a class='struct' href='nalgebra/geometry/point/struct.PointBase.html' title='nalgebra::geometry::point::PointBase'>PointBase</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, S&gt;&gt; for <a class='struct' href='nalgebra/core/unit/struct.Unit.html' title='nalgebra::core::unit::Unit'>Unit</a>&lt;<a class='struct' href='num_complex/struct.Complex.html' title='num_complex::Complex'>Complex</a>&lt;N&gt;&gt; <span class='where fmt-newline'>where N: <a class='trait' href='alga/general/real/trait.Real.html' title='alga::general::real::Real'>Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S: <a class='trait' href='nalgebra/core/storage/trait.OwnedStorage.html' title='nalgebra::core::storage::OwnedStorage'>OwnedStorage</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;S::<a class='trait' href='nalgebra/core/storage/trait.Storage.html' title='nalgebra::core::storage::Storage'>Alloc</a>: <a class='trait' href='nalgebra/core/allocator/trait.OwnedAllocator.html' title='nalgebra::core::allocator::OwnedAllocator'>OwnedAllocator</a>&lt;N, <a class='struct' href='nalgebra/core/dimension/struct.U2.html' title='nalgebra::core::dimension::U2'>U2</a>, <a class='struct' href='nalgebra/core/dimension/struct.U1.html' title='nalgebra::core::dimension::U1'>U1</a>, S&gt;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
