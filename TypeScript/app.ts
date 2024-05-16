
class AppContext {
    static envs: string[];
    static apps: IdGroupHttpModel[];
    static labels: string[];
    static features: string[];

    static selectedProcess: string;
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



