if string.sub(Message, 1, 6) == '/echo ' then
    Rmessage = string.gsub(Message, '/echo ', '')
end

if string.sub(Message, 1, 5) == '/说 ' then
    Rmessage = string.gsub(Message, '/说 ', '')
end
