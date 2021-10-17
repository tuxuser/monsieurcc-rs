import React, { useState, useEffect, ChangeEvent } from "react";
import { RouteComponentProps } from 'react-router-dom';

import RecipeDataService from "../services/RecipeService";
import IRecipeData from "../types/Recipe";

interface RouterProps { // type for `match.params`
  id: string; // must be type `string` since value comes from the URL
}

type Props = RouteComponentProps<RouterProps>;

const Recipe: React.FC<Props> = (props: Props) => {
  const initialRecipeState = {
    id: null,
    name: "",
    description: "",
    published: false
  };
  const [currentRecipe, setCurrentRecipe] = useState<IRecipeData>(initialRecipeState);
  const [message, setMessage] = useState<string>("");

  const getRecipe = (id: string) => {
    RecipeDataService.get(id)
      .then(response => {
        setCurrentRecipe(response.data);
        console.log(response.data);
      })
      .catch(e => {
        console.log(e);
      });
  };

  useEffect(() => {
    getRecipe(props.match.params.id);
  }, [props.match.params.id]);

  const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setCurrentRecipe({ ...currentRecipe, [name]: value });
  };

  const updateRecipe = () => {
    RecipeDataService.update(currentRecipe.id, currentRecipe)
      .then(response => {
        console.log(response.data);
        setMessage("The recipe was updated successfully!");
      })
      .catch(e => {
        console.log(e);
      });
  };

  const deleteRecipe = () => {
    RecipeDataService.remove(currentRecipe.id)
      .then(response => {
        console.log(response.data);
        props.history.push("/recipes");
      })
      .catch(e => {
        console.log(e);
      });
  };

  return (
    <div>
      {currentRecipe ? (
        <div className="edit-form">
          <h4>Recipe</h4>
          <form>
            <div className="form-group">
              <label htmlFor="title">Title</label>
              <input
                type="text"
                className="form-control"
                id="title"
                name="title"
                value={currentRecipe.name}
                onChange={handleInputChange}
              />
            </div>

            <div className="form-group">
              <label>
                <strong>Custom:</strong>
              </label>
              {currentRecipe.is_custom ? "Yes" : "No"}
            </div>
          </form>

          <button className="badge badge-danger mr-2" onClick={deleteRecipe}>
            Delete
          </button>

          <button
            type="submit"
            className="badge badge-success"
            onClick={updateRecipe}
          >
            Update
          </button>
          <p>{message}</p>
        </div>
      ) : (
        <div>
          <br />
          <p>Please click on a Recipe...</p>
        </div>
      )}
    </div>
  );
};

export default Recipe;