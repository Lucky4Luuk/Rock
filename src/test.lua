local t = 0;

function rock.load()
    print("Called from rock.load!")
    rock.graphics.clear(1, 0, 0, 1);
end

function rock.update(dt)
    -- print("Called from rock.update!")
    -- print("dt: " .. tostring(dt))
    t = t + dt;
end

function rock.draw()
    -- print("Called from rock.draw!")
    local c = math.fmod(t, 6.28) -- 2 PI
    rock.graphics.clear(math.sin(c), math.cos(c), 1, 1);
end
