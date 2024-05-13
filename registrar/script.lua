local publish_domain = get("publish-input-domain")
local publish_tld = get("publish-input-tld")
local publish_ip = get("publish-input-ip")
local publish_done = get("done-1")

local update_key = get("update-input-key")
local update_tld = get("update-input-ip")
local update_done = get("done-2")

local delete_key = get("delete-input-key")
local delete_done = get("done-3")

local result = get("result")

coroutine.wrap(function()
	local res = fetch({
		url = "http://api.buss.lol/tlds",
		method = "GET",
		headers = { ["Content-Type"] = "application/json" },
	})

	local tld_list = table.concat(res, ", ")
	get("tlds").set_content("Available TLDs: " .. tld_list)
end)()

publish_done.on_click(function()
	local body = "{"
		.. '"tld": "'
		.. publish_tld.get_content()
		.. '", '
		.. '"name": "'
		.. publish_domain.get_content()
		.. '", '
		.. '"ip": "'
		.. publish_ip.get_content()
		.. '"'
		.. "}"

	local res = fetch({
		url = "http://api.buss.lol/domain",
		method = "POST",
		headers = { ["Content-Type"] = "application/json" },
		body = body,
	})

	print(res)
	if res and res.status then
		if res.status == 429 then
			result.set_content("Failed due to ratelimit.")
		else
			result.set_content("Failed due to error: " .. res.status)
		end
	elseif res and res.secret_key then
		result.set_content(
			"Success! Your key is: "
				.. res.secret_key
				.. "\n\nMAKE SURE TO SAVE IT! You will need it to update/delete your domain."
		)
	else
		result.set_content("Failed due to unknown error.")
	end
end)

-- btn.on_click(function()
--     print("clicked!")
-- end)

-- get("input").on_submit(function(content)
--     print(content)
-- end)

-- get("input").on_input(function(content)
--     print(content)
-- end)

-- get("textarea").on_input(function(content)
--     print(content)
-- end)

-- get("futurelink").set_href("https://www.duckduckgo.com/")

-- coroutine.wrap(function()
-- 	local res = fetch({
-- 		url = "http://127.0.0.1:3000/",
-- 		method = "POST",
-- 		headers = { ["Content-Type"] = "application/json" },
-- 		body = '{ "test": 3 }',
-- 	})

-- 	print("hlelo", { hello = true })
-- end)()
