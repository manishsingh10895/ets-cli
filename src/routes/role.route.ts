
import {Express, Router} from "express";

export default class RoleRoute {
    constructor(app: Express) {
        let _router = Router();

        app.use('/v1/role', _router);
    }
}
        