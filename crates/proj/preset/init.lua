---@class TemplateStore
---@field render_template fun(name:string,params:table):string
---@field add_template fun(name:string,template:string)

---@alias RouteHandler fun(params:table):string

---@class Router
---@field get fun(route:string,handler:RouteHandler)
---@field post fun(route:string,handler:RouteHandler)
---@field delete fun(route:string,handler:RouteHandler)
---@field put fun(route:string,handler:RouteHandler)
---@field patch fun(route:string,handler:RouteHandler)
---@field head fun(route:string,handler:RouteHandler)

---@class Proj
---@field templates TemplateStore
---@field router Router

---@type Proj
---@diagnostic disable-next-line:lowercase-global
proj = proj or {}

proj.templates.add_template("hello", "Hello {{user.name}} {{user.surname}}")

proj.router.get("hello/{name}/{surname}", function(params)
	return proj.templates.render_template("hello", { user = params })
end)
