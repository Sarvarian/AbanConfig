pub const TEMPLATE_OS_PROCESS_MODULES_H: &str = r#"
// This file generated by Aban Config.
// Include and ue subroutines in this file just once.
// Including this file more then once can give you linker errors. (I think!)

#pragma once
#ifndef ABAN_OS_PROCESS_MODULES_H
#define ABAN_OS_PROCESS_MODULES_H

int os_init_modules(int argc, char **argv)
{

    {{add_modules_inits}}
    
    return 0;
}

void os_exit_modules()
{
    
    {{add_modules_exits}}
    
}

#endif // ABAN_OS_PROCESS_MODULES_H
"#;