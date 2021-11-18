pub const TEMPLATE_OS_PROCESS_MODULES_H: &str = r#"
// This file generated by Aban Config.
// Include and ue subroutines in this file just once.
// Including this file more then once can give you linker errors. (I think!)

#pragma once
#ifndef ABAN_OS_PROCESS_H
#define ABAN_OS_PROCESS_H

static char os_process_error_module_name[255] = "\0";

int os_init_modules(int argc, char **argv)
{
    int err = 0;

    {{add_modules_inits}}

    // Should be 0 for success.
    return err;
}

void os_exit_modules()
{

    {{add_modules_exits}}
}

char *get_error_module_name()
{
    return os_process_error_module_name;
}

#endif // ABAN_OS_PROCESS_H
"#;

pub const TEMPLATE_OS_ADD_MODULE_INIT: &str = r#"
    err = {{module_name}}_init(argc, argv);
    if (err != 0)
    {
        os_process_error_module_name = {{module_name}};
        return err;
    }
"#;

pub const TEMPLATE_OS_ADD_MODULE_EXIT: &str = r#"
    {{module_name}}_exit();
"#;
