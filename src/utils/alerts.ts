import Swal from 'sweetalert2'

const Toast = Swal.mixin({
    toast: true,
    position: 'top-end',
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    background: 'var(--color-background-secondary)',
    color: 'var(--color-text)',
    didOpen: (toast) => {
        toast.addEventListener('mouseenter', Swal.stopTimer)
        toast.addEventListener('mouseleave', Swal.resumeTimer)
    }
})

const Question = Swal.mixin({
    background: 'var(--color-background-secondary)',
    color: 'var(--color-text)',
    showCancelButton: true,
    confirmButtonColor: 'var(--color-quinary)',
    cancelButtonColor: 'var(--color-tertiary)',
    input: 'text',
    iconHtml: '',
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