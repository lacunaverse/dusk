const q = (id: string) => document.body.querySelector('#' + id) as HTMLInputElement;
const el = (id: string) => document.body.querySelector('#' + id) as HTMLElement;

const stopCode = q('stopcode');
const url = q('url');

el('send').addEventListener('click', async () => {
    const urlValue = url.value;
    const stopCodeValue = stopCode.value;

    if (!urlValue.startsWith('https://')) {
        url.setAttribute('invalid', 'true');
        return;
    }

    if (stopCodeValue.length < 4 || stopCodeValue.length > 20) {

    }

    try {
        const resp = await fetch('/add', {
            method: "post",
            body: JSON.stringify({ stopcode: stopCodeValue, link: urlValue })
        })

        const data = await resp.json()

        if (data.error) {
            displayText(data.error)
        } else if (data.id) {
            displaySuccess(data.id)
        }
    } catch (err) {
        displayText(err)
    }
});

const deleteID = q('delete-id');
const deleteSC = q('delete-stopcode');
el('delete-send').addEventListener('click', async () => {
    const idValue = deleteID.value;
    const stopCodeValue = deleteSC.value;

    if (stopCodeValue.length < 4 || stopCodeValue.length > 20) {

    }

    try {
        const resp = await fetch('/delete', {
            method: "DELETE",
            body: JSON.stringify({ stopcode: stopCodeValue, id: idValue })
        })

        if (resp.status == 204) {
            displayText('Successfully deleted shortlink.', false)
        } else {
            const data = await resp.json()

            if (data.error) {
                displayText(data.error)
            }
        }

    } catch (err) {
        displayText(err)
    }
})

const $msg = q('msg');
function displayText(text: string, error = true) {
    removeKids();
    $msg.style.display = '';
    error ? $msg.classList.add('error') : '';
    $msg.innerText = text;
}

function removeKids() {
    Array.from($msg.children).forEach(i => i.remove());
    $msg.innerText = '';
}

function buildEl(type: string, text: string, attr?: Object) {
    const $el = document.createElement(type);
    $el.innerText = text;
    if (attr) {
        Object.entries(attr).forEach(([k, v]) => {
            $el.setAttribute(k, v);
        });
    }

    return $el;
}

function displaySuccess(id: string) {
    $msg.classList.remove('error');

    removeKids();
    $msg.style.display = '';
    $msg.appendChild(buildEl('span', 'Your new link has been created at '))

    const link = buildEl('a', `${window.origin}/l/${id}`, { href: `${window.origin}/l/${id}` });
    $msg.appendChild(link);

    $msg.appendChild(buildEl('span', `, with the id ${id}. Make sure to save the ID and stopcode in a safe place. Otherwise, you won't be able to delete your shortened link.`))
}