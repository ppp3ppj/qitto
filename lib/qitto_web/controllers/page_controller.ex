defmodule QittoWeb.PageController do
  use QittoWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
