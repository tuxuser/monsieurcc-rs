import React, { useState, ChangeEvent } from "react";
import RecipeDataService from "../services/RecipeService";
import IRecipeData from '../types/Recipe';

const AddRecipe: React.FC = () => {
  const initialRecipeState = {
    id: null,
    name: "",
    description: "",
    published: false
  };
  const [recipe, setRecipe] = useState<IRecipeData>(initialRecipeState);
  const [submitted, setSubmitted] = useState<boolean>(false);

  const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setRecipe({ ...recipe, [name]: value });
  };

  const saveRecipe = () => {
    var data = {
      title: recipe.name,
      name: recipe.name,
    };

    RecipeDataService.create(data)
      .then(response => {
        setRecipe({
          id: response.data.id,
          name: response.data.name,
          is_custom: response.data.is_custom
        });
        setSubmitted(true);
        console.log(response.data);
      })
      .catch(e => {
        console.log(e);
      });
  };

  const newRecipe = () => {
    setRecipe(initialRecipeState);
    setSubmitted(false);
  };

  return (
    <div className="submit-form">
      {submitted ? (
        <div>
          <h4>You submitted successfully!</h4>
          <button className="btn btn-success" onClick={newRecipe}>
            Add
          </button>
        </div>
      ) : (
        <div>
          <div className="form-group">
            <label htmlFor="title">Title</label>
            <input
              type="text"
              className="form-control"
              id="title"
              required
              value={recipe.name}
              onChange={handleInputChange}
              name="name"
            />
          </div>

          <button onClick={saveRecipe} className="btn btn-success">
            Submit
          </button>
        </div>
      )}
    </div>
  );
};

export default AddRecipe;