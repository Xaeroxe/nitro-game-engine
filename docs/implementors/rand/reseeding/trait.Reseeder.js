(function() {var implementors = {};
implementors["alga"] = [];
implementors["chrono"] = [];
implementors["enum_primitive"] = [];
implementors["libc"] = [];
implementors["nalgebra"] = [];
implementors["ncollide"] = [];
implementors["num"] = [];
implementors["rand"] = [];
implementors["sdl2"] = [];
implementors["serde"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
