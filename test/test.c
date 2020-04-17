#include <windows.h>
#include <stdio.h>

typedef void *(*napi_addon_register_func)(void *nv,
                                               void *exports);

typedef struct {
  int nm_version;
  unsigned int nm_flags;
  const char* nm_filename;
  napi_addon_register_func nm_register_func;
  const char* nm_modname;
  void* nm_priv;
  void* reserved[4];
} napi_module;

__declspec(dllexport) void napi_module_register(napi_module *module) {
    printf("in napi_module_register, module = %p\n", module);
    printf("func = %p\n", module->nm_register_func);
    printf("calling...\n", module->nm_register_func);
    module->nm_register_func(0, 0);
    printf("called.");
}

__declspec(dllexport) void napi_create_object() {
    printf("in napi_create_object!\n");
}

__declspec(dllexport) void napi_create_string_utf8() {
    printf("in napi_create_string_utf8!\n");
}

int main() {
    printf("address of napi_module_register = %p\n", napi_module_register);

    printf("Before LoadLibrary...\n");
    HINSTANCE h = LoadLibraryA("wallet.dll");
    printf("After LoadLibrary, result = %p...\n", h);
    return 0;
}