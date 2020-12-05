local t = 0;
local triangle = nil;

function rock.load()
    --You can use this graphics function anywhere, because
    --it merely modifies the underlying pipeline state's clear colour
    rock.graphics.clear(1, 0, 0, 1)

    triangle = rock.graphics.mesh()
end

function rock.update(dt)
    t = t + dt
end

function rock.draw()
    rock.graphics.clear(math.sin(t), math.cos(t), 1, 1)
    rock.graphics.draw(triangle)
    rock.graphics.draw(triangle)
end
