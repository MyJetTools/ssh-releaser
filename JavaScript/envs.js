class Envs {
    static refresh() {
        let html = this.render(this.getSelected());
        document.getElementById("left-panel").innerHTML = html;
    }
    static render(selected) {
        let result = "";
        for (let itm of AppContext.envs) {
            let featureBadge = "";
            if (itm.feature) {
                featureBadge = '<div><span class="badge text-bg-primary">' + itm.feature + '</span></div>';
            }
            if (itm.id == selected) {
                result += `<div class="btn btn-secondary" style="width:100%">${itm.id}${featureBadge}</div>`;
            }
            else {
                result += `<div data-id="${itm.id}" class="btn btn-outline-secondary" style="width:100%" onclick="Envs.select(this)">${itm.id}${featureBadge}</div>`;
            }
        }
        return result;
    }
    static getSelected() {
        return localStorage.getItem("selectedEnv");
    }
    static select(itm) {
        let id = itm.getAttribute("data-id");
        localStorage.setItem("selectedEnv", id);
        this.refresh();
        Apps.request(id);
    }
}
//# sourceMappingURL=envs.js.map