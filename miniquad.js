// Miniquad JS bindings for WebGL
let gl;
let shaders = new Map();
let buffers = new Map();
let textures = new Map();
let programs = new Map();
let vertexArrays = new Map();

let nextId = 1;
function getId() {
    return nextId++;
}

window.miniquad_add_plugin = function(plugin) {
    // Plugin system placeholder
};

window.miniquad = {
    wasm_exports: null,
    
    env: {
        console_log: function(ptr, len) {
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, len);
            let string = new TextDecoder().decode(array);
            console.log(string);
        },
        
        console_error: function(ptr, len) {
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, len);
            let string = new TextDecoder().decode(array);
            console.error(string);
        },
        
        set_cursor_grab: function() {},
        show_mouse: function() {},
        
        canvas_width: function() {
            return Math.floor(window.innerWidth);
        },
        
        canvas_height: function() {
            return Math.floor(window.innerHeight);
        },
        
        dpi_scale: function() {
            return window.devicePixelRatio || 1.0;
        },
        
        high_dpi: function() {
            return window.devicePixelRatio > 1.0 ? 1 : 0;
        },
        
        setup_canvas_size: function(width, height) {
            let canvas = document.getElementById('glcanvas');
            canvas.width = width;
            canvas.height = height;
        },
        
        now: function() {
            return Date.now() / 1000.0;
        },
        
        rand: function() {
            return Math.random();
        },
        
        fs_load_file: function(ptr, len) {
            return 0;
        },
        
        fs_write_file: function(ptr1, len1, ptr2, len2) {
            // Stub
        },
        
        run_animation_loop: function() {
            // Animation loop stub - the actual loop is handled by main()
        },
        
        request_animation_frame: function(callback) {
            return window.requestAnimationFrame(callback);
        },
        
        cancel_animation_frame: function(id) {
            window.cancelAnimationFrame(id);
        },
        
        glfwGetTime: function() {
            return Date.now() / 1000.0;
        },
        
        // Raw OpenGL function names (some versions of macroquad expect these)
        glClear: function(mask) { gl.clear(mask); },
        glClearColor: function(r, g, b, a) { gl.clearColor(r, g, b, a); },
        glViewport: function(x, y, width, height) { gl.viewport(x, y, width, height); },
        glEnable: function(cap) { gl.enable(cap); },
        glDisable: function(cap) { gl.disable(cap); },
        glBlendFunc: function(sfactor, dfactor) { gl.blendFunc(sfactor, dfactor); },
        glDepthFunc: function(func) { gl.depthFunc(func); },
        
        glCreateShader: function(shader_type) {
            let shader = gl.createShader(shader_type);
            let id = getId();
            shaders.set(id, shader);
            return id;
        },
        
        glShaderSource: function(shader_id, count, ptr, lengths) {
            // For simplicity, assuming single source string
            let length = new Int32Array(window.miniquad.wasm_exports.memory.buffer, lengths, 1)[0];
            let source_ptr = new Uint32Array(window.miniquad.wasm_exports.memory.buffer, ptr, 1)[0];
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, source_ptr, length);
            let source = new TextDecoder().decode(array);
            let shader = shaders.get(shader_id);
            gl.shaderSource(shader, source);
        },
        
        glCompileShader: function(shader_id) {
            let shader = shaders.get(shader_id);
            gl.compileShader(shader);
        },
        
        glGetShaderiv: function(shader_id, pname, params) {
            let shader = shaders.get(shader_id);
            let result = gl.getShaderParameter(shader, pname);
            let view = new Int32Array(window.miniquad.wasm_exports.memory.buffer, params, 1);
            view[0] = result ? 1 : 0;
        },
        
        glCreateProgram: function() {
            let program = gl.createProgram();
            let id = getId();
            programs.set(id, program);
            return id;
        },
        
        glAttachShader: function(program_id, shader_id) {
            let program = programs.get(program_id);
            let shader = shaders.get(shader_id);
            gl.attachShader(program, shader);
        },
        
        glLinkProgram: function(program_id) {
            let program = programs.get(program_id);
            gl.linkProgram(program);
        },
        
        glGetProgramiv: function(program_id, pname, params) {
            let program = programs.get(program_id);
            let result = gl.getProgramParameter(program, pname);
            let view = new Int32Array(window.miniquad.wasm_exports.memory.buffer, params, 1);
            view[0] = result ? 1 : 0;
        },
        
        glUseProgram: function(program_id) {
            let program = programs.get(program_id);
            gl.useProgram(program);
        },
        
        glGetAttribLocation: function(program_id, ptr) {
            let name = "";
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr);
            for (let i = 0; array[i] !== 0; i++) {
                name += String.fromCharCode(array[i]);
            }
            let program = programs.get(program_id);
            return gl.getAttribLocation(program, name);
        },
        
        glGetUniformLocation: function(program_id, ptr) {
            let name = "";
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr);
            for (let i = 0; array[i] !== 0; i++) {
                name += String.fromCharCode(array[i]);
            }
            let program = programs.get(program_id);
            let location = gl.getUniformLocation(program, name);
            if (location === null) return -1;
            let id = getId();
            programs.set("uniform_" + id, location);
            return id;
        },
        
        glGenBuffers: function(n, out_ptr) {
            let view = new Uint32Array(window.miniquad.wasm_exports.memory.buffer, out_ptr, n);
            for (let i = 0; i < n; i++) {
                let buffer = gl.createBuffer();
                let id = getId();
                buffers.set(id, buffer);
                view[i] = id;
            }
        },
        
        glBindBuffer: function(target, buffer_id) {
            let buffer = buffers.get(buffer_id);
            gl.bindBuffer(target, buffer);
        },
        
        glBufferData: function(target, size, ptr, usage) {
            let data = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, size);
            gl.bufferData(target, data, usage);
        },
        
        glVertexAttribPointer: function(index, size, type, normalized, stride, offset) {
            gl.vertexAttribPointer(index, size, type, normalized, stride, offset);
        },
        
        glEnableVertexAttribArray: function(index) {
            gl.enableVertexAttribArray(index);
        },
        
        glUniform1f: function(location_id, v0) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform1f(location, v0);
        },
        
        glUniform2f: function(location_id, v0, v1) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform2f(location, v0, v1);
        },
        
        glUniform3f: function(location_id, v0, v1, v2) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform3f(location, v0, v1, v2);
        },
        
        glUniform4f: function(location_id, v0, v1, v2, v3) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform4f(location, v0, v1, v2, v3);
        },
        
        glUniform1i: function(location_id, v0) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform1i(location, v0);
        },
        
        glUniformMatrix4fv: function(location_id, count, transpose, ptr) {
            let location = programs.get("uniform_" + location_id);
            let data = new Float32Array(window.miniquad.wasm_exports.memory.buffer, ptr, count * 16);
            gl.uniformMatrix4fv(location, transpose, data);
        },
        
        glGenTextures: function(n, out_ptr) {
            let view = new Uint32Array(window.miniquad.wasm_exports.memory.buffer, out_ptr, n);
            for (let i = 0; i < n; i++) {
                let texture = gl.createTexture();
                let id = getId();
                textures.set(id, texture);
                view[i] = id;
            }
        },
        
        glBindTexture: function(target, texture_id) {
            let texture = textures.get(texture_id);
            gl.bindTexture(target, texture);
        },
        
        glTexParameteri: function(target, pname, param) {
            gl.texParameteri(target, pname, param);
        },
        
        glTexImage2D: function(target, level, internalformat, width, height, border, format, type, ptr) {
            if (ptr === 0) {
                gl.texImage2D(target, level, internalformat, width, height, border, format, type, null);
            } else {
                let size = width * height * 4; // Assuming RGBA for now
                let data = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, size);
                gl.texImage2D(target, level, internalformat, width, height, border, format, type, data);
            }
        },
        
        glActiveTexture: function(texture) {
            gl.activeTexture(texture);
        },
        
        glDrawArrays: function(mode, first, count) {
            gl.drawArrays(mode, first, count);
        },
        
        glDrawElements: function(mode, count, type, indices) {
            gl.drawElements(mode, count, type, indices);
        },
        
        init_webgl: function() {
            let canvas = document.getElementById('glcanvas');
            gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
            if (!gl) {
                throw new Error("WebGL is not supported");
            }
            return 0;
        },
        
        // WebGL functions
        gl_clear_color: function(r, g, b, a) {
            gl.clearColor(r, g, b, a);
        },
        
        gl_clear: function(mask) {
            gl.clear(mask);
        },
        
        gl_viewport: function(x, y, width, height) {
            gl.viewport(x, y, width, height);
        },
        
        gl_enable: function(cap) {
            gl.enable(cap);
        },
        
        gl_disable: function(cap) {
            gl.disable(cap);
        },
        
        gl_blend_func: function(sfactor, dfactor) {
            gl.blendFunc(sfactor, dfactor);
        },
        
        gl_depth_func: function(func) {
            gl.depthFunc(func);
        },
        
        gl_create_shader: function(shader_type) {
            let shader = gl.createShader(shader_type);
            let id = getId();
            shaders.set(id, shader);
            return id;
        },
        
        gl_shader_source: function(shader_id, ptr, len) {
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, len);
            let source = new TextDecoder().decode(array);
            let shader = shaders.get(shader_id);
            gl.shaderSource(shader, source);
        },
        
        gl_compile_shader: function(shader_id) {
            let shader = shaders.get(shader_id);
            gl.compileShader(shader);
        },
        
        gl_get_shader_parameter: function(shader_id, pname) {
            let shader = shaders.get(shader_id);
            return gl.getShaderParameter(shader, pname) ? 1 : 0;
        },
        
        gl_get_shader_info_log: function(shader_id, ptr, max_len) {
            let shader = shaders.get(shader_id);
            let log = gl.getShaderInfoLog(shader) || "";
            let encoder = new TextEncoder();
            let encoded = encoder.encode(log);
            let view = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, Math.min(encoded.length, max_len - 1));
            view.set(encoded.slice(0, Math.min(encoded.length, max_len - 1)));
            view[Math.min(encoded.length, max_len - 1)] = 0;
            return encoded.length;
        },
        
        gl_create_program: function() {
            let program = gl.createProgram();
            let id = getId();
            programs.set(id, program);
            return id;
        },
        
        gl_attach_shader: function(program_id, shader_id) {
            let program = programs.get(program_id);
            let shader = shaders.get(shader_id);
            gl.attachShader(program, shader);
        },
        
        gl_link_program: function(program_id) {
            let program = programs.get(program_id);
            gl.linkProgram(program);
        },
        
        gl_get_program_parameter: function(program_id, pname) {
            let program = programs.get(program_id);
            return gl.getProgramParameter(program, pname) ? 1 : 0;
        },
        
        gl_get_program_info_log: function(program_id, ptr, max_len) {
            let program = programs.get(program_id);
            let log = gl.getProgramInfoLog(program) || "";
            let encoder = new TextEncoder();
            let encoded = encoder.encode(log);
            let view = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, Math.min(encoded.length, max_len - 1));
            view.set(encoded.slice(0, Math.min(encoded.length, max_len - 1)));
            view[Math.min(encoded.length, max_len - 1)] = 0;
            return encoded.length;
        },
        
        gl_use_program: function(program_id) {
            let program = programs.get(program_id);
            gl.useProgram(program);
        },
        
        gl_get_attrib_location: function(program_id, ptr, len) {
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, len);
            let name = new TextDecoder().decode(array);
            let program = programs.get(program_id);
            return gl.getAttribLocation(program, name);
        },
        
        gl_get_uniform_location: function(program_id, ptr, len) {
            let array = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, len);
            let name = new TextDecoder().decode(array);
            let program = programs.get(program_id);
            let location = gl.getUniformLocation(program, name);
            if (location === null) return -1;
            let id = getId();
            // Store uniform locations with a special prefix to distinguish from other objects
            programs.set("uniform_" + id, location);
            return id;
        },
        
        gl_create_buffer: function() {
            let buffer = gl.createBuffer();
            let id = getId();
            buffers.set(id, buffer);
            return id;
        },
        
        gl_bind_buffer: function(target, buffer_id) {
            let buffer = buffers.get(buffer_id);
            gl.bindBuffer(target, buffer);
        },
        
        gl_buffer_data: function(target, ptr, size, usage) {
            let data = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, size);
            gl.bufferData(target, data, usage);
        },
        
        gl_vertex_attrib_pointer: function(index, size, type, normalized, stride, offset) {
            gl.vertexAttribPointer(index, size, type, normalized, stride, offset);
        },
        
        gl_enable_vertex_attrib_array: function(index) {
            gl.enableVertexAttribArray(index);
        },
        
        gl_uniform1f: function(location_id, v0) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform1f(location, v0);
        },
        
        gl_uniform2f: function(location_id, v0, v1) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform2f(location, v0, v1);
        },
        
        gl_uniform3f: function(location_id, v0, v1, v2) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform3f(location, v0, v1, v2);
        },
        
        gl_uniform4f: function(location_id, v0, v1, v2, v3) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform4f(location, v0, v1, v2, v3);
        },
        
        gl_uniform1i: function(location_id, v0) {
            let location = programs.get("uniform_" + location_id);
            gl.uniform1i(location, v0);
        },
        
        gl_uniform_matrix4fv: function(location_id, count, transpose, ptr) {
            let location = programs.get("uniform_" + location_id);
            let data = new Float32Array(window.miniquad.wasm_exports.memory.buffer, ptr, count * 16);
            gl.uniformMatrix4fv(location, transpose, data);
        },
        
        gl_create_texture: function() {
            let texture = gl.createTexture();
            let id = getId();
            textures.set(id, texture);
            return id;
        },
        
        gl_bind_texture: function(target, texture_id) {
            let texture = textures.get(texture_id);
            gl.bindTexture(target, texture);
        },
        
        gl_tex_parameteri: function(target, pname, param) {
            gl.texParameteri(target, pname, param);
        },
        
        gl_tex_image2d: function(target, level, internalformat, width, height, border, format, type, ptr) {
            if (ptr === 0) {
                gl.texImage2D(target, level, internalformat, width, height, border, format, type, null);
            } else {
                let size = width * height * 4; // Assuming RGBA for now
                let data = new Uint8Array(window.miniquad.wasm_exports.memory.buffer, ptr, size);
                gl.texImage2D(target, level, internalformat, width, height, border, format, type, data);
            }
        },
        
        gl_active_texture: function(texture) {
            gl.activeTexture(texture);
        },
        
        gl_draw_arrays: function(mode, first, count) {
            gl.drawArrays(mode, first, count);
        },
        
        gl_draw_elements: function(mode, count, type, offset) {
            gl.drawElements(mode, count, type, offset);
        },
        
        gl_delete_shader: function(shader_id) {
            let shader = shaders.get(shader_id);
            gl.deleteShader(shader);
            shaders.delete(shader_id);
        },
        
        gl_delete_program: function(program_id) {
            let program = programs.get(program_id);
            gl.deleteProgram(program);
            programs.delete(program_id);
        },
        
        gl_delete_buffer: function(buffer_id) {
            let buffer = buffers.get(buffer_id);
            gl.deleteBuffer(buffer);
            buffers.delete(buffer_id);
        },
        
        gl_delete_texture: function(texture_id) {
            let texture = textures.get(texture_id);
            gl.deleteTexture(texture);
            textures.delete(texture_id);
        },
    }
};