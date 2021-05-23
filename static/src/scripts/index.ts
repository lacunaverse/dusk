const id = document.body.querySelector("#link-id") as HTMLInputElement;
const passcode = document.body.querySelector(
    "#link-passcode"
) as HTMLInputElement;
const url = document.body.querySelector("#link-url") as HTMLInputElement;
const results = document.body.querySelector("#results") as HTMLDivElement;

interface ResponseOk {
    status: "ok";
    shortlink: string;
}

export function build<K extends keyof HTMLElementTagNameMap>(
    type: string | K,
    attributes?: { [key: string]: string } | string,
    ...children: (HTMLElement | Element | string)[]
) {
    let element = document.createElement(type);

    if (attributes && typeof attributes == "string") {
        element.textContent = attributes;
    } else if (attributes && typeof attributes == "object" && attributes.text) {
        element.textContent = attributes.text;
    }

    if (typeof attributes == "object" && attributes != null) {
        Object.keys(attributes).forEach(item => {
            if (item == "text") return;
            if (element.hasAttribute(item) || item in element) {
                element.setAttribute(item, attributes[item]);
            } else if (item == "class") {
                element.classList.add(...attributes[item].split(" "));
            } else if (item.startsWith("data_")) {
                element.dataset[item.replace("data_", "")] = attributes[item];
            }
        });
    }

    if (children.length > 0) {
        children.forEach(i => {
            if (typeof i == "string") {
                element.appendChild(document.createTextNode(i));
            } else {
                element.appendChild(i);
            }
        });
    }

    return element;
}

document.body.querySelector("#link-create")?.addEventListener("click", () => {
    const forms = {
        id: id.value,
        passcode: passcode.value,
        url: url.value,
    };

    Object.keys(forms).forEach(i =>
        document.body.querySelector(`#link-${i}`)?.classList.remove("invalid")
    );

    results.childNodes.forEach(i => i.remove());

    const isValid = Object.values(forms).find(i => i.length <= 0);
    if (isValid == undefined) {
        const serialized = JSON.stringify(forms);
        fetch("/new", {
            body: serialized,
            method: "POST",
            headers: { "Content-Type": "application/json" },
        })
            .then(r => {
                if (r.status != 200) {
                    throw r.statusText;
                } else return r.json();
            })
            .then(data => {
                console.log(data);
                const { shortlink } = data;

                const url = new URL(
                    window.location.protocol +
                        window.location.host +
                        "/link/" +
                        shortlink
                ).toString();

                const p = build(
                    "p",
                    "Your new link is available at ",
                    build("a", {
                        href: url,
                        text: url,
                    })
                );

                results.appendChild(p);
            })
            .catch(err => {});
    } else {
        Object.entries(forms).forEach(([key, value]) => {
            if (value.length == 0) {
                document.body
                    .querySelector(`#link-${key}`)
                    ?.classList.add("invalid");
            }
        });
    }
});
