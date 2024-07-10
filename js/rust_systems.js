const SYSTEM_PREFIX = '__system_';

export function aframe_system_def(rust_obj) {
    const def = {
        rust_system: rust_obj,
    }
    const proto = Object.getPrototypeOf(rust_obj);
    const names = Object.getOwnPropertyNames(proto);
    const tasks = names
        .filter((name) => name.lastIndexOf(SYSTEM_PREFIX) === 0 && typeof proto[name] == 'function')
        .map((name) => [proto[name], name.slice(SYSTEM_PREFIX.length)])
    ;
    for (const task of tasks) {
        const key = task[1];
        const rust_method = task[0];
        def[key] = function (...args) {
            return rust_method.call(rust_obj, this, ...args);
        }
    }
    console.log(def);
    return def;
}