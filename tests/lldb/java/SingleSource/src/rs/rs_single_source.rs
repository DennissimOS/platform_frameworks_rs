#pragma version(1)
#pragma rs java_package_name(com.android.rs.singlesource)
#pragma rs_fp_full

// global allocation used for void kernel
rs_allocation global_alloc;

static void check_in()
{
    // debugger check point
    return;
}

float __attribute__((kernel)) kernel_1(float a)
{
    // square
    return a * a;
}

float __attribute__((kernel)) kernel_2(float a, float b)
{
    // product
    return a * b;
}

void __attribute__((kernel)) void_kernel_1(uint32_t x)
{
    // allocation[x] = x
    rsSetElementAt_float(global_alloc, (float)x, x);
}

void script_invoke_1(rs_allocation out, rs_allocation in1, rs_allocation in2)
{
    // invoke kernel taking one argument
    rsForEach(kernel_1, out, in1);

    check_in();

    // invoke kernel taking two arguments
    rsForEach(kernel_2, out, in1, in2);

    check_in();
}

void script_invoke_2()
{
    // invoke kernel that takes no arguments and no return type
    rs_script_call_t options = {
        .strategy=RS_FOR_EACH_STRATEGY_DONT_CARE,
        .xStart=0,
        .xEnd=4
    };
    rsForEachWithOptions(void_kernel_1, &options);

    check_in();
}
