const id = document.body.querySelector("#link-id") as HTMLInputElement;
const passcode = document.body.querySelector(
    "#link-passcode"
) as HTMLInputElement;
const url = document.body.querySelector("#link-url") as HTMLInputElement;

document.body.querySelector("#link-create")?.addEventListener("click", () => {
    const forms = {
        id: id.value,
        passcode: passcode.value,
        url: url.value,
    };

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
            })
            .catch(err => {});
    } else {
    }
});
