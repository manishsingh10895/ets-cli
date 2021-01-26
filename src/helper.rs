use inflector::Inflector;

pub fn gen_route_content(name: &str) -> (String, String) {
    let t_name: String = name.to_title_case() + "Route";
    let content = format!(
        r#"
import {{Express, Router}} from "express";

export default class {t_name} {{
    constructor(app: Express) {{
        let _router = Router();

        app.use('/v1/{name}', _router);
    }}
}}
        "#,
        t_name = t_name,
        name = name
    );

    (content, format!("{}.route.ts", name))
}

pub fn gen_service_content(name: &str) -> (String, String) {
    let t_name: String = name.to_title_case() + "Service";
    let content = format!(
        r#"
import logger from "../helpers/logger";
import {{ AppError }} from "../infra/app-error";

class {t_name} {{
    
}}

export default new {t_name}();
        "#,
        t_name = t_name,
    );

    (content, format!("{}.service.ts", name))
}

pub fn gen_controller_content(name: &str) -> (String, String) {
    let t_name: String = name.to_title_case() + "Controller";
    let content = format!(
        r#"
import {{ Request, Response }} from 'express';
import {{ validationResult }} from 'express-validator';
import logger from '../helpers/logger';
import NResponse from '../services/response.service';
import {{ Errors }} from '../infra/messages';
import {{ AppError }} from '../infra/app-error';

export default class {t_name} {{
    
}}
        "#,
        t_name = t_name,
    );

    (content, format!("{}.controller.ts", name))
}

pub fn gen_model(name: &str) -> (String, String) {
    let cap_name: String = format!("{}", name.to_title_case().clone());
    let t_name: String = format!("I{}", cap_name.clone());
    let t_doc_name: String = format!("I{}", cap_name.clone() + "Document");
    let t_schema_name: String = cap_name.clone() + "Schema";

    let content = format!(
        r#"
import {{ Schema, Document, model, Model, Mongoose, Types }} from "mongoose";
export interface {t_doc_name} extends Document, {t_name} {{

}}

export interface {t_name} {{
    _id: any,
    createdAt: Date,
    updatedAt: Date,
}}

export const {t_schema_name}: Schema = new Schema({{
    createdAt: {{ type: Date, default: new Date() }},
    updatedAt: {{ type: Date, default: new Date() }}
}}) 

export const {cap_name}: Model<{t_doc_name}> = model<{t_doc_name}>('User', {t_schema_name});
        "#,
        t_name = t_name,
        t_doc_name = t_doc_name,
        cap_name = cap_name,
        t_schema_name = t_schema_name
    );

    (content, format!("{}.model.ts", name))
}

pub fn gen_request_schema(name: &str) -> (String, String) {
    let t_name: String = name.to_title_case() + "RequestSchema";
    let content = format!(
        r#"
import {{ checkSchema, ValidationSchema }} from 'express-validator';

export const {t_name}: ValidationSchema = {{

}}
        "#,
        t_name = t_name,
    );

    (content, format!("{}.request-schema.ts", name))
}

pub fn gen_middleware_content(name: &str) -> (String, String) {
    let t_name: String = name.to_title_case() + "Middleware";
    let content = format!(
        r#"
/**
 * Use only static methods here
 * 
 */

import logger from "../helpers/logger";
import {{ Request, Response }} from 'express';
import NResponse from "../services/response.service";

export default class {t_name} {{

}}
        "#,
        t_name = t_name,
    );

    (content, format!("{}.middleware.ts", name))
}
