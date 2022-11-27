import Sweetalert from 'sweetalert2'

const Toast = Sweetalert.mixin({
    toast: true,
    position: 'bottom-end',
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    background: 'var(--color-background-secondary)',
    color: 'var(--color-text)',
    didOpen: (toast) => {
        toast.addEventListener('mouseenter', Sweetalert.stopTimer)
        toast.addEventListener('mouseleave', Sweetalert.resumeTimer)
    }
})

const Question = Sweetalert.mixin({
    background: 'var(--color-background-secondary)',
    color: 'var(--color-text)',
    showCancelButton: true,
    confirmButtonColor: 'var(--color-quinary)',
    cancelButtonColor: 'var(--color-tertiary)',
    input: 'text',
    iconHtml: '',
})

const Confirm = Sweetalert.mixin({
    background: 'var(--color-background-secondary)',
    color: 'var(--color-text)',
    showCancelButton: true,
    confirmButtonColor: 'var(--color-quinary)',
    cancelButtonColor: 'var(--color-tertiary)',
})

export const toast = async (title: string, text: string = "", icon: 'success' | 'error' | 'warning' | 'info' | 'question' = 'success') => {
    return Toast.fire({
        icon,
        title,
        text
    })
}

export const question = async (title: string, text: string = "", placeholder: string = "") => {
    console.log(placeholder)
    return Question.fire({
        title,
        text,
        inputValue: placeholder
    })
}

export const awaitConfirm = async (title: string, text: string = "") => {
    return Confirm.fire({
        title,
        text,
        icon: 'warning'
    })
}