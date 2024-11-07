--[[
	Attempts to set a property on the given instance.
]]

local Packages = script.Parent.Parent.Parent.Packages
local Log = require(Packages.Log)
local RbxDom = require(Packages.RbxDom)
local Error = require(script.Parent.Error)

local function setProperty(instance, propertyName, value)
	if propertyName == "StyledProperties" then

		-- Enum can't be serialised as numbers within styled properties so we need to convert enum strings into enums.
		for propName, propValue in value do
			local match = typeof(propValue) == "string" and string.match(propValue, "^Enum%.(.+)")
			if not match then continue end
			local enum = Enum

			local err = false
			for _, segment in match:split(".") do
				local ok = pcall(function() enum = enum[segment] end)
				if not ok then
					err = true
					break
				end
			end

			if not err then
				value[propName] = enum
			end
		end

		instance:SetProperties(value)
		return true
	end
	
	local descriptor = RbxDom.findCanonicalPropertyDescriptor(instance.ClassName, propertyName)

	-- We can skip unknown properties; they're not likely reflected to Lua.
	--
	-- A good example of a property like this is `Model.ModelInPrimary`, which
	-- is serialized but not reflected to Lua.
	if descriptor == nil then
		Log.trace("Skipping unknown property {}.{}", instance.ClassName, propertyName)

		return true
	end

	if descriptor.scriptability == "None" or descriptor.scriptability == "Read" then
		return false,
			Error.new(Error.UnwritableProperty, {
				className = instance.ClassName,
				propertyName = propertyName,
			})
	end

	local writeSuccess, err = descriptor:write(instance, value)

	if not writeSuccess then
		if err.kind == RbxDom.Error.Kind.Roblox and err.extra:find("lacking permission") then
			return false,
				Error.new(Error.LackingPropertyPermissions, {
					className = instance.ClassName,
					propertyName = propertyName,
				})
		end

		return false,
			Error.new(Error.OtherPropertyError, {
				className = instance.ClassName,
				propertyName = propertyName,
			})
	end

	return true
end

return setProperty
