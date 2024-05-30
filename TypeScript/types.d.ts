declare const $;


interface ReleaseAppsHttpModel {
    ids: IdGroupHttpModel[];
    labels: string[];
}


interface IdGroupHttpModel {
    category: string;
    ids: string[];
}

interface IEnvironment {
    id: String,
    feature?: String;
}