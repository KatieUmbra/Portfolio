export class BlogPost {
    constructor(
        public id: number,
        public creator: string,
        public content: string,
        public description: string,
        public title: string,
        public creation: Date | string,
        public likes: number) {
    }
}

export class BlogPostData {
    constructor(
        public title: string,
        public description: string,
        public content: string
    ) {}
}
