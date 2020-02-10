#include <stdbool.h>
#include <stdio.h>

// #include "daqiface.h"

ssize_t daqiface_get_analog_input_scan(char* buf_ptr, size_t buf_size)
{
    for(size_t i = 0; i < buf_size; ++i)
	*(buf_ptr + i) = (char)i;

    return 0;
}

ssize_t daqiface_set_digital_output(size_t digital_output_index, bool state)
{
    printf("c: Sent state %d to digital out %d\n", state, digital_output_index);
    
    return 0;
}


ssize_t daqiface_get_digital_input(size_t digital_input_index, bool* state)
{
    // test state
    *state = true;
    
    printf("c: Got state %d from digital in %d\n", *state, digital_input_index);

    return 0;
}
