import axios from "axios";

export default axios.create({
  baseURL: "http://localhost",
  headers: {
    "Content-type": "application/json"
  }
});