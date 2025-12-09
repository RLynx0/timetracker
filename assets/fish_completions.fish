function __timetrack_subcommands
    timetrack help | awk '
        /^\w/ { c = 0 }
        /^Commands:/ { c = 1; next }
        !/^$/ && c {
            com = $1
            gsub(/^\s*\w+\s*/, "", $0)
            printf "%s\t\'%s\'\n", com, $0
        }
    '
end
function __timetrack_attendance_types
    timetrack list-attendance-types | awk -F ' *: *' '
    /^->/ {
        gsub(/^->\s*/, "", $1)
        print $1 "\t\'" $2 "\'"
    }'
end
function __timetrack_activities
    timetrack activity list
end

complete -c timetrack -f -a "(__timetrack_subcommands)"
complete -c timetrack \
    -rfl attendance \
    -n '__fish_seen_subcommand_from start' \
    -a "(__timetrack_attendance_types)"
complete -c timetrack \
    -rfs a \
    -n '__fish_seen_subcommand_from start' \
    -a "(__timetrack_attendance_types)"
complete -c timetrack -rfl verbose \
    -n '__fish_seen_subcommand_from start'
complete -c timetrack -rfs v \
    -n '__fish_seen_subcommand_from start'
