class AppContext {
    static hideBackground() {
        let background = document.getElementById("background");
        background.classList.remove("visible");
        background.classList.add("hidden");
    }
}
setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        Envs.refresh();
        Apps.init();
    });
}, 100);
setInterval(function () {
    if (AppContext.selectedProcess) {
        $.ajax({ url: "/api/release/logs", data: { id: AppContext.selectedProcess } }).then(function (data) {
            let doc = document.getElementById("content");
            doc.innerHTML = data.html;
            doc.scrollTo(0, doc.scrollHeight);
            if (data.finished == true) {
                AppContext.selectedProcess = undefined;
            }
        });
    }
}, 1000);
function backgroundClick() {
    let background = document.getElementById("background");
    background.classList.remove("visible");
    background.classList.add("hidden");
}
//# sourceMappingURL=app.js.map