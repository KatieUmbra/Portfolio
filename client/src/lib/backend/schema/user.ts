export class Claims {
    constructor(
        public username: string,
        public rank: number,
        public iat: number | Date,
        public exp: number | Date
    ) {}
}

export class UserData {
    constructor(
        public username: string,
        public display_name: string,
        public email: String,
        public password: string
    ) {}
}

export class LoginData {
    constructor(
        public username: string,
        public password: string
    ) {}
}
