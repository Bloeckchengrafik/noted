export const debounce = (func: Function, delay: number) => {
    let debounceTimer: any;
    return function (...args: any) {
        // @ts-ignore
        const context = this
        clearTimeout(debounceTimer)
        debounceTimer
            = setTimeout(() => func.apply(context, args), delay)
    }
}

export const throttle = (func: Function, limit: number) => {
    let inThrottle: boolean
    return function (...args: any) {
        // @ts-ignore
        const context = this
        if (!inThrottle) {
            func.apply(context, args)
            inThrottle = true
            setTimeout(() => inThrottle = false, limit)
        }
    }
}

export const afterInactivity = (func: Function, delay: number) => {
    let timeout: any
    return function (...args: any) {
        console.log('afterInactivity')
        // @ts-ignore
        const context = this
        clearTimeout(timeout)
        timeout = setTimeout(() => func.apply(context, args), delay)
    }
}