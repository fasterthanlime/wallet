
#define STUB(NAME) void NAME() { \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
}

STUB(napi_module_register);
STUB(napi_create_object);
STUB(napi_create_string_utf8);

