(function() {var implementors = {};
implementors["chrono"] = [];
implementors["nalgebra"] = [];
implementors["ncollide"] = [];
implementors["num"] = [];
implementors["rand"] = [];
implementors["sdl2"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
