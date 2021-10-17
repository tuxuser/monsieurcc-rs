import React, { useState, useEffect, ChangeEvent } from "react";
import RecipeDataService from "../services/RecipeService";
import { Link } from "react-router-dom";
import IRecipeData from '../types/Recipe';

const RecipesList: React.FC = () => {
  const [recipes, setRecipes] = useState<Array<IRecipeData>>([]);
  const [currentRecipe, setCurrentRecipe] = useState<IRecipeData | null>(null);
  const [currentIndex, setCurrentIndex] = useState<number>(-1);
  const [searchTitle, setSearchTitle] = useState<string>("");

  useEffect(() => {
    retrieveRecipes();
  }, []);

  const onChangeSearchTitle = (e: ChangeEvent<HTMLInputElement>) => {
    const searchTitle = e.target.value;
    setSearchTitle(searchTitle);
  };

  const retrieveRecipes = () => {
    RecipeDataService.getAll()
      .then(response => {
        setRecipes(response.data);
        console.log(response.data);
      })
      .catch(e => {
        console.log(e);
      });
  };

  const refreshList = () => {
    retrieveRecipes();
    setCurrentRecipe(null);
    setCurrentIndex(-1);
  };

  const setActiveRecipe = (recipe: IRecipeData, index: number) => {
    setCurrentRecipe(recipe);
    setCurrentIndex(index);
  };

  const removeAllRecipes = () => {
    RecipeDataService.removeAll()
      .then(response => {
        console.log(response.data);
        refreshList();
      })
      .catch(e => {
        console.log(e);
      });
  };

  const findByTitle = () => {
    RecipeDataService.findByTitle(searchTitle)
      .then(response => {
        setRecipes(response.data);
        setCurrentRecipe(null);
        setCurrentIndex(-1);
        console.log(response.data);
      })
      .catch(e => {
        console.log(e);
      });
  };

  return (
    <div className="list row">
      <div className="col-md-8">
        <div className="input-group mb-3">
          <input
            type="text"
            className="form-control"
            placeholder="Search by title"
            value={searchTitle}
            onChange={onChangeSearchTitle}
          />
          <div className="input-group-append">
            <button
              className="btn btn-outline-secondary"
              type="button"
              onClick={findByTitle}
            >
              Search
            </button>
          </div>
        </div>
      </div>
      <div className="col-md-6">
        <h4>Recipes List</h4>

        <ul className="list-group">
          {recipes &&
            recipes.map((recipe, index) => (
              <li
                className={
                  "list-group-item " + (index === currentIndex ? "active" : "")
                }
                onClick={() => setActiveRecipe(recipe, index)}
                key={index}
              >
                {recipe.name}
              </li>
            ))}
        </ul>

        <button
          className="m-3 btn btn-sm btn-danger"
          onClick={removeAllRecipes}
        >
          Remove All
        </button>
      </div>
      <div className="col-md-6">
        {currentRecipe ? (
          <div>
            <h4>Recipe</h4>
            <div>
              <label>
                <strong>Name:</strong>
              </label>{" "}
              {currentRecipe.name}
            </div>
            <div>
              <label>
                <strong>Custom:</strong>
              </label>{" "}
              {currentRecipe.is_custom ? "Yes" : "No"}
            </div>

            <Link
              to={"/recipes/" + currentRecipe.id}
              className="badge badge-warning"
            >
              Edit
            </Link>
          </div>
        ) : (
          <div>
            <br />
            <p>Please click on a Recipe...</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default RecipesList;