import {EwtsConverter} from 'ewts'
import "toastify-js/src/toastify.css"
import Toastify from 'toastify-js'
import './style.scss'

const converter = new EwtsConverter()

const inputArea = document.querySelector<HTMLTextAreaElement>('.inputarea')!
const convertedResultEl = document.querySelector<HTMLParagraphElement>('.convertedResult')!
const copyResultBtn = document.querySelector<HTMLButtonElement>('.copyResult')!

function updateConvertedResultEl() {
  const val = inputArea.value
  const convertedResultElText = converter.ewtsToUnicode(val)
  convertedResultEl.textContent = convertedResultElText

  const valIsEmpty = convertedResultElText.length === 0
  convertedResultEl.classList.toggle('empty', valIsEmpty)
  copyResultBtn.disabled = valIsEmpty

  //if (convertedResultEl.scrollHeight > (convertedResultEl.clientHeight * 1.5)) {
  //  convertedResultEl.style.fontSize = '1em'
  //} else {
  //  convertedResultEl.removeAttribute('style')
  //}
}

function listenAndUpdate() {
  inputArea.addEventListener('input', () => {
    updateConvertedResultEl()
    convertedResultEl.scrollTo({top: 999999, behavior: 'smooth'})
  })
}

updateConvertedResultEl()
listenAndUpdate()


copyResultBtn.addEventListener('click', () => {
  const val = convertedResultEl.textContent

  if (!val || !val.trim()) {
    return;
  }

  navigator.clipboard.writeText(val)
    .then(() => {
      Toastify({
        text: "Text copied",
        duration: 2000,
        newWindow: true,
        gravity: "top",
        position: "right",
        style: {
          background: "linear-gradient(to right, var(--color_orange), var(--color_pink))",
          fontSize: '1.2em',
        },
      }).showToast();
    })
})

