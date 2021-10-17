import http from "../http-common";
import IRecipeData from "../types/Recipe";

const getAll = () => {
  return http.get("/admin/recipes/overview");
};

const get = (id: any) => {
  return http.get(`/admin/recipes/${id}`);
};

const create = (data: IRecipeData) => {
  return http.post("/admin/recipes", data);
};

const update = (id: any, data: IRecipeData) => {
  return http.put(`/admin/recipes/${id}`, data);
};

const remove = (id: any) => {
  return http.delete(`/admin/recipes/${id}`);
};

const removeAll = () => {
  return http.delete(`/admin/recipes`);
};

const findByTitle = (title: string) => {
  return http.get(`/admin/recipes?title=${title}`);
};

const RecipeService = {
  getAll,
  get,
  create,
  update,
  remove,
  removeAll,
  findByTitle,
};

export default RecipeService;