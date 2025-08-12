//currently there is a layer of abstraction. The pages will container a pointer to other pages that
//are related i.e a record could be greater than 16KB so we must use multiple pages to store it. A
//pointer of 0 means the end has been reached. This can also be useful for table indexes and maybe
//table schema.

struct header{
    file_type:      String::from(".mdbf"),
    version:        u8,
    page_size:      u16,
}

struct database_page{
    page_type: PageType,
}

enum PageType{
    data                { content: [field_type] },
    table_index         { record_pointers: [u32] },
    table_schema        { table_index_pointer: u32, format: [field_type] },
    database_table_list { table_name: str, table_pointers: [u32]},
}


struct table_schema_content{
    table_name:             str,
    table_index_pointer:    u32,
    format:                 [field_type],
}

enum field_type{
    integer:    i64,
    string      {length: length_type, content: str},
    float:      f64,
    blob:       [u8],
}


enum length_type{
    fixed,
    non-fixed   {size: u32},
}
