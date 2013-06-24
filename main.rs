use core::libc::{c_void, size_t};
use lua51::*;

extern fn lua_alloc(_ud: *c_void,
					p: *c_void,
					_osize: size_t,
					nsize: size_t) -> *c_void {
	unsafe {
		use core::libc::{realloc, free};

		if nsize == 0 {
			println(fmt!("freeing alloc of %? bytes", _osize));
			free(p);
			return ptr::null();
		} else {
			println(fmt!("reallocing %? -> %? bytes", _osize, nsize));
			return realloc(p, nsize);
		}
	}
}

fn main() {
	let state = LuaState::new(lua_alloc, ptr::null());

	let rv = state.L_dostring("function foo (x,y) return x+y end");

	state.getglobal("foo");

	state.push(5);
	state.push(3);

	state.call(2, SomeResults(1));

	println(fmt!("Result: %?", state.tointeger(-1)));
}