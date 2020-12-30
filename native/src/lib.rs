use neon::prelude::*;
use neon::register_module;
use num_cpus;

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

fn fibonacci(n: f64) -> f64 {
    if n <= 2.0 { return 1.0 }
    return fibonacci(n-1.0) + fibonacci(n-2.0);
}

fn  neon_fibo(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value();
    Ok(cx.number(fibonacci(n)))
}

register_module!(mut m, { 
    m.export_function("threadCount", thread_count);
    m.export_function("rustFibonacci", neon_fibo);
    Ok(())
});
