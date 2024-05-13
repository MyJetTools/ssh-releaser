
class Envs {
    static refresh() {
        let html = this.render(this.getSelected());
        document.getElementById("left-panel").innerHTML = html;
    }

    static render(selected: string): string {
        let result = "";

        for (let itm of AppContext.envs) {
            if (itm == selected) {
                result += '<div class="btn btn-secondary" style="width:100%">' + itm + "</div>";
            } else {
                result += '<div data-id="' + itm + '" class="btn btn-outline-secondary" style="width:100%" onclick="Envs.select(this)">' + itm + "</div>";
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








