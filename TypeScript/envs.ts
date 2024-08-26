
class Envs {
    static refresh() {
        let html = this.render(this.getSelected());
        document.getElementById("env-select-panel").innerHTML = html;
    }

    static render(selected: string): string {

        let result = "";

        let selectedProduct = AppContext.getSelectedProduct();

        for (let itm of AppContext.getEnvs(selectedProduct)) {

            let featureBadge = "";

            if (itm.features) {

                let odd = false;
                for (let feature of itm.features) {

                    let badgeType = getBadgeType(odd);
                    featureBadge += '<div><span class="badge ' + badgeType + '">' + feature + '</span></div>';

                    odd = !odd;
                }

            }

            if (itm.id == selected) {
                result += `<div class="btn btn-secondary" style="width:100%">${itm.id}${featureBadge}</div>`;
            } else {
                result += `<div data-id="${itm.id}" class="btn btn-outline-secondary" style="width:100%" onclick="Envs.select(this)">${itm.id}${featureBadge}</div>`;
            }

        }

        return result;
    }

    static getSelected(): string {
        return localStorage.getItem("selectedEnv");
    }

    static select(itm) {
        let id = itm.getAttribute("data-id");
        localStorage.setItem("selectedEnv", id);
        this.refresh();
        Apps.request(id);
    }



}


function getBadgeType(odd: boolean): string {
    if (odd) {
        return "text-bg-warning";
    }

    return "text-bg-primary"

}




