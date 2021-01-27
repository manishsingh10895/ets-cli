
import { Schema, Document, model, Model, Mongoose, Types } from "mongoose";
export interface IRoleDocument extends Document, IRole {

}

export interface IRole {
    _id: any,
    createdAt: Date,
    updatedAt: Date,
}

export const RoleSchema: Schema = new Schema({
    createdAt: { type: Date, default: new Date() },
    updatedAt: { type: Date, default: new Date() }
}) 

export const Role: Model<IRoleDocument> = model<IRoleDocument>('User', RoleSchema);
        