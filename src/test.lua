local t = 0
local models = {}

function rock.load()
    print("test!")

    --You can use this graphics function anywhere, because
    --it merely modifies the underlying pipeline state's clear colour
    -- rock.graphics.clear(1, 0, 0, 1)

    local meshes, transforms = rock.graphics.load_mesh("sphere.glb", "glb")
    for i=1,#meshes do
        models[i] = {meshes[i], transforms[i]}
    end
end

function rock.update(dt)
    t = t + dt
    local tmp = rock.math.quat_euler(3.14/4,0,0)
    for i=1,#models do
        models[i][2]:setRotation(tmp)
        -- models[i][2]:setScale(rock.math.vec3(0.075, 0.075, 0.075))
    end
end

function rock.draw()
    -- rock.graphics.clear(math.sin(t), math.cos(t), 1, 1)
    for i=1,#models do
        rock.graphics.draw(models[i][1], models[i][2])
    end
end
