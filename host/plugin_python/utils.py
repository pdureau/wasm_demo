from wasmtime import Store
from wasmtime.component import (
    Instance,
    Func,
    ExportIndex,
    Component,
    Instance,
    Linker,
    LinkerInstance,
)


def get_export_func(
    store: Store, instance: Instance, interface: str, function: str
) -> Func | None:
    interface_index: ExportIndex | None = instance.get_export_index(store, interface)
    if not interface_index:
        return None
    function_index: ExportIndex | None = instance.get_export_index(
        store, function, instance=interface_index
    )
    if not function_index:
        return None
    return instance.get_func(store, function_index)


def _import_host_func(linker: Linker, interface: str, function: str, callable):
    instance: LinkerInstance = linker.root().add_instance(interface)
    instance.add_func(function, callable)


def init_instance(store: Store, component_path: str, host: dict) -> Instance:
    # Linker: Component linking system for resolving imports and connecting multiple components.
    linker = Linker(store.engine)
    linker.add_wasip2()
    for interface, function in host.items():
        for function, callable in function.items():
            _import_host_func(linker, interface, function, callable)
    # Component: Compiled WebAssembly component containing validated and optimized code
    component: Component = Component.from_file(store.engine, component_path)
    return linker.instantiate(store, component)
