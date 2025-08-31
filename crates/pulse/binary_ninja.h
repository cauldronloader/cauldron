struct String
{
    char const* data;
};

struct RTTIStringSpan
{
    char const* data;
    uint32_t length;
};

enum RTTIKind : uint8_t
{
    Atom = 0x0,
    Pointer = 0x1,
    Container = 0x2,
    Enum = 0x3,
    Compound = 0x4,
    EnumFlags = 0x5,
    POD = 0x6,
    EnumBitSet = 0x7
};

enum RTTIFlags : uint8_t
{
    RTTIFactorySource = 0x2,
    FactoryManagerSource = 0x4
};

struct RTTI __packed
{
    uint32_t id;
    enum RTTIKind kind;
    enum RTTIFlags flags;
};

struct RTTIIter
{
    struct RTTI* container_type;
    void* container;
    uint32_t user_data;
};

struct __base(RTTI, 0) RTTIAtom
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    uint16_t size;
    uint8_t alignment;
    bool simple;
    char const* type_name;
    struct RTTIAtom* base_type;
    bool (* fn_from_string)(struct RTTIStringSpan& value, void const* result);
    bool (* fn_to_string)(void const* value, struct String& string);
    void* fn_unk30;
    void (* fn_copy)(void* in, void const* out);
    bool (* fn_equals)(void const* left, void const* right);
    void* (* fn_constructor)(struct RTTI* rtti, void* result);
    void* (* fn_destructor)(struct RTTI* rtti, void* result);
    bool (* fn_serialize)(void* in, void* out, bool swap_endian);
    bool (* fn_deserialize)(void* in, void* out);
    int32_t (* fn_get_serialized_size)(void const* in);
    void (* fn_range_check)(void const* obj, char const* min, char const* max);
    struct RTTI* representation_type;
};

enum `RTTICompound::Attribute::Flags` : uint16_t
{
    ATTR_DONT_SERIALIZE_BINARY = 0x2,
    ARRT_VALID_FLAG_MASK = 0xdeb
};

struct `RTTICompound::Attribute``
{
    struct RTTI* type;
    uint16_t offset;
    enum `RTTICompound::Attribute::Flags` flags;
    char const* name;
    void (* fn_get)(void const* compound, void* value);
    void (* fn_set)(void* compound, void const* value);
    char const* min;
    char const* max;
};

struct __base(`RTTICompound::Attribute`, 0) `RTTICompound::OrderedAttribute``
{
    __inherited struct RTTI* `RTTICompound::Attribute::type`;
    __inherited uint16_t `RTTICompound::Attribute::offset`;
    __inherited enum RTTICompound::Attribute::Flags `RTTICompound::Attribute::flags`;
    __inherited char const* `RTTICompound::Attribute::name`;
    __inherited void (* `RTTICompound::Attribute::fn_get`)(void const* compound, void* value);
    __inherited void (* `RTTICompound::Attribute::fn_set`)(void* compound, void const* value);
    __inherited char const* `RTTICompound::Attribute::min`;
    __inherited char const* `RTTICompound::Attribute::max`;
    struct RTTI* parent;
    char const* group;
};

struct `RTTICompound::Base`
{
    struct RTTI* type;
    uint32_t offset;
};

struct `RTTICompound::MessageHandler`
{
    struct RTTI* message;
    void (* handler)(void*, void*);
};

struct `RTTICompound::MessageOrderEntry`
{
    bool before;
    struct RTTI* message;
    struct RTTI* compound;
};

struct __base(RTTI, 0) RTTICompound
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    uint8_t bases_len;
    uint8_t attributes_len;
    uint8_t message_handelers_len;
    uint8_t message_order_entries_len;
    uint8_t unk0;
    uint16_t version;
    uint32_t size;
    uint16_t alignment;
    uint16_t serialize_flags;
    void* (* fn_constructor)(struct RTTI* rtti, void* result);
    void* (* fn_destructor)(struct RTTI* rtti, void* result);
    bool (* fn_from_string)(void* value, struct String& string);
    void* unk1;
    bool (* fn_to_string)(void const* value, struct String& string);
    char const* type_name;
    struct RTTI* next_type;
    struct RTTI* prev_type;
    struct `RTTICompound::Base`* bases;
    struct `RTTICompound::Attribute`* attributes;
    struct `RTTICompound::MessageHandler`* message_handlers;
    struct `RTTICompound::MessageOrderEntry`* message_order_entries;
    struct RTTI* (* fn_get_symbol_group)();
    struct RTTI* pod_optimised_type;
    struct `RTTICompound::OrderedAttribute`* ordered_attributes;
    uint32_t ordered_attributes_len;
    struct `RTTICompound::MessageHandler` message_read_binary;
    uint32_t message_read_binary_offset;
    uint32_t unk;
};

struct `RTTIContainer::Data`
{
    char const* type_name;
    uint16_t size;
    uint8_t alignment;
    bool simple;
    bool associative;
    void* (* fn_constructor)(struct RTTI* rtti, void* result);
    void* (* fn_destructor)(struct RTTI* rtti, void* result);
    bool (* fn_resize)(struct RTTI* rtti, void* object, int32_t new_size);
    void* fn_unk0;
    bool (* fn_remove)(struct RTTI* rtti, void* object, int32_t index);
    int32_t (* fn_len)(struct RTTI* rtti, void const* object);
    struct RTTIIter (* fn_iter_start)(struct RTTI* rtti, void const*);
    struct RTTIIter (* fn_iter_end)(struct RTTI* rtti, void const*);
    void (* fn_iter_next)(struct RTTIIter& iter);
    void* (* fn_iter_deref)(struct RTTIIter& iter);
    bool (* fn_iter_is_valid)(struct RTTIIter& iter);
    void* fn_unk1;
    void* fn_unk2;
    struct RTTIIter (* fn_add_item)(struct RTTI* rtti, void* container, void* value);
    struct RTTIIter (* fn_add_empty)(struct RTTI* rtti, void* container);
    bool (* fn_clear)(struct RTTI* rtti, void* container);
    bool (* fn_to_string)(void const* container, struct RTTI* rtti, struct String& string);
    bool (* fn_from_string)(struct RTTIStringSpan& string, struct RTTI* rtti, void const* container);
};

struct __base(RTTI, 0) RTTIContainer
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    bool has_pointers;
    struct RTTI* item;
    struct `RTTIContainer::Data`* container;
    char const* type_name;
};

struct `RTTIPointer::Data`
{
    char const* type_name;
    uint16_t size;
    uint8_t alignment;
    void* (* fn_constructor)(struct RTTI* rtti, void* result);
    void* (* fn_destructor)(struct RTTI* rtti, void* result);
    void* (* fn_get)(struct RTTI* rtti, void const* object);
    bool (* fn_set)(struct RTTI* rtti, void** object, void* value);
    void (* fn_copy)(void** left, void** right);
};

struct __base(RTTI, 0) RTTIPointer
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    bool has_pointers;
    struct RTTI* item;
    struct `RTTIPointer::Data`* pointer;
    char const* type_name;
};

struct `RTTIEnum::Value`
{
    int32_t value;
    char const* name;
    char const* aliases[0x4];
};

struct __base(RTTI, 0) RTTIEnum
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    uint8_t size;
    uint8_t alignment;
    uint16_t values_len;
    char const* type_name;
    struct `RTTIEnum::Value`* values;
    struct RTTI* pod_optimised_type;
};

struct RTTIEnumBitSet
{
    struct RTTI* type;
    char const* type_name;
};

struct __base(RTTI, 0) RTTIPod
{
    __inherited uint32_t `RTTI::id`;
    __inherited enum RTTIKind `RTTI::kind`;
    __inherited enum RTTIFlags `RTTI::flags`;
    uint32_t size;
};

