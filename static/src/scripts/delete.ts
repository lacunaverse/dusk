import { build } from "./index";

const id = document.body.querySelector("#link-id") as HTMLInputElement;
const passcode = document.body.querySelector(
    "#link-passcode"
) as HTMLInputElement;
const results = document.body.querySelector("#results") as HTMLDivElement;

document.body.querySelector("#link-delete")?.addEventListener("click", () => {
    const forms = {
        id: id.value,
        passcode: passcode.value,
    };

    Object.keys(forms).forEach(i =>
        document.body.querySelector(`#link-${i}`)?.classList.remove("invalid")
    );

    results.childNodes.forEach(i => i.remove());

    const isValid = Object.values(forms).find(i => i.length <= 0);
    if (isValid == undefined) {
        const serialized = JSON.stringify(forms);
        console.warn(serialized);
        fetch("/delete", {
            body: serialized,
            method: "POST",
            headers: { "Content-Type": "application/json" },
        })
            .then(r => {
                console.error(r);
                if (r.status != 200) {
                    throw r.statusText;
                } else return r.json();
            })
            .then(data => {
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
            .catch(err => {
                console.log(err);
                results.appendChild(
                    build(
                        "p",
                        "Something went wrong :(",
                        build("p", "Please try again.")
                    )
                );
            });
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
